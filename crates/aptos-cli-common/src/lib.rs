//! Common modules for creating an Aptos CLI command.

mod summary;
pub mod utils;
pub use summary::*;

use aptos_cli_config::{
    error::{CliError, CliTypedResult},
    utils::read_from_file,
    CliConfig,
};
use aptos_crypto::{
    ed25519::{Ed25519PrivateKey, Ed25519PublicKey},
    x25519, PrivateKey, ValidCryptoMaterial, ValidCryptoMaterialStringExt,
};
use aptos_rest_client::{Client, Transaction};
use aptos_sdk::{
    move_types::{
        ident_str,
        language_storage::{ModuleId, TypeTag},
    },
    transaction_builder::TransactionFactory,
    types::LocalAccount,
};
use aptos_types::transaction::{
    authenticator::AuthenticationKey, ScriptFunction, TransactionPayload,
};
use aptos_types::{account_address::AccountAddress, chain_id::ChainId};
use clap::{ArgEnum, Parser};
use std::{
    collections::BTreeMap,
    fmt::{Debug, Display, Formatter},
    path::{Path, PathBuf},
    str::FromStr,
};
use url::Url;
use utils::{get_chain_id, get_sequence_number};

pub const DEFAULT_REST_URL: &str = "https://fullnode.devnet.aptoslabs.com";
pub const DEFAULT_FAUCET_URL: &str = "https://faucet.devnet.aptoslabs.com";

/// Types of Keys used by the blockchain
#[derive(ArgEnum, Clone, Copy, Debug)]
pub enum KeyType {
    /// Ed25519 key used for signing
    Ed25519,
    /// X25519 key used for network handshakes and identity
    X25519,
}

impl Display for KeyType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            KeyType::Ed25519 => "ed25519",
            KeyType::X25519 => "x25519",
        };
        write!(f, "{}", str)
    }
}

impl FromStr for KeyType {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "ed25519" => Ok(KeyType::Ed25519),
            "x25519" => Ok(KeyType::X25519),
            _ => Err("Invalid key type"),
        }
    }
}

#[derive(Debug, Parser)]
pub struct ProfileOptions {
    /// Profile to use from config
    #[clap(long, default_value = "default")]
    pub profile: String,
}

impl ProfileOptions {
    pub fn account_address(&self) -> CliTypedResult<AccountAddress> {
        if let Some(profile) = CliConfig::load_profile(&self.profile)? {
            if let Some(account) = profile.account {
                return Ok(account);
            }
        }

        Err(CliError::ConfigNotFoundError(self.profile.clone()))
    }
}

impl Default for ProfileOptions {
    fn default() -> Self {
        Self {
            profile: "default".to_string(),
        }
    }
}

/// Types of encodings used by the blockchain
#[derive(ArgEnum, Clone, Copy, Debug)]
pub enum EncodingType {
    /// Binary Canonical Serialization
    BCS,
    /// Hex encoded e.g. 0xABCDE12345
    Hex,
    /// Base 64 encoded
    Base64,
}

impl EncodingType {
    /// Encodes `Key` into one of the `EncodingType`s
    pub fn encode_key<Key: ValidCryptoMaterial>(
        &self,
        name: &'static str,
        key: &Key,
    ) -> CliTypedResult<Vec<u8>> {
        Ok(match self {
            EncodingType::Hex => hex::encode_upper(key.to_bytes()).into_bytes(),
            EncodingType::BCS => bcs::to_bytes(key).map_err(|err| CliError::BCS(name, err))?,
            EncodingType::Base64 => base64::encode(key.to_bytes()).into_bytes(),
        })
    }

    /// Loads a key from a file
    pub fn load_key<Key: ValidCryptoMaterial>(
        &self,
        name: &'static str,
        path: &Path,
    ) -> CliTypedResult<Key> {
        self.decode_key(name, read_from_file(path)?)
    }

    /// Decodes an encoded key given the known encoding
    pub fn decode_key<Key: ValidCryptoMaterial>(
        &self,
        name: &'static str,
        data: Vec<u8>,
    ) -> CliTypedResult<Key> {
        match self {
            EncodingType::BCS => bcs::from_bytes(&data).map_err(|err| CliError::BCS(name, err)),
            EncodingType::Hex => {
                let hex_string = String::from_utf8(data).unwrap();
                Key::from_encoded_string(hex_string.trim())
                    .map_err(|err| CliError::UnableToParse(name, err.to_string()))
            }
            EncodingType::Base64 => {
                let string = String::from_utf8(data).unwrap();
                let bytes = base64::decode(string.trim())
                    .map_err(|err| CliError::UnableToParse(name, err.to_string()))?;
                Key::try_from(bytes.as_slice()).map_err(|err| {
                    CliError::UnableToParse(name, format!("Failed to parse key {:?}", err))
                })
            }
        }
    }
}

impl Default for EncodingType {
    fn default() -> Self {
        EncodingType::Hex
    }
}

impl Display for EncodingType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            EncodingType::BCS => "bcs",
            EncodingType::Hex => "hex",
            EncodingType::Base64 => "base64",
        };
        write!(f, "{}", str)
    }
}

impl FromStr for EncodingType {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "hex" => Ok(EncodingType::Hex),
            "bcs" => Ok(EncodingType::BCS),
            "base64" => Ok(EncodingType::Base64),
            _ => Err("Invalid encoding type"),
        }
    }
}

/// An insertable option for use with prompts.
#[derive(Clone, Copy, Debug, Parser)]
pub struct PromptOptions {
    /// Assume yes for all yes/no prompts
    #[clap(long, group = "prompt_options")]
    pub assume_yes: bool,
    /// Assume no for all yes/no prompts
    #[clap(long, group = "prompt_options")]
    pub assume_no: bool,
}

impl PromptOptions {
    pub fn yes() -> Self {
        Self {
            assume_yes: true,
            assume_no: false,
        }
    }
}

/// An insertable option for use with encodings.
#[derive(Debug, Default, Parser)]
pub struct EncodingOptions {
    /// Encoding of data as `base64`, `bcs`, or `hex`
    #[clap(long, default_value_t = EncodingType::Hex)]
    pub encoding: EncodingType,
}

#[derive(Debug, Parser)]
pub struct PublicKeyInputOptions {
    /// Public key input file name
    #[clap(long, group = "public_key_input", parse(from_os_str))]
    public_key_file: Option<PathBuf>,
    /// Public key encoded in a type as shown in `encoding`
    #[clap(long, group = "public_key_input")]
    public_key: Option<String>,
}

impl ExtractPublicKey for PublicKeyInputOptions {
    fn extract_public_key(
        &self,
        encoding: EncodingType,
        _profile: &str,
    ) -> CliTypedResult<Ed25519PublicKey> {
        if let Some(ref file) = self.public_key_file {
            encoding.load_key("--public-key-file", file.as_path())
        } else if let Some(ref key) = self.public_key {
            let key = key.as_bytes().to_vec();
            encoding.decode_key("--public-key", key)
        } else {
            Err(CliError::CommandArgumentError(
                "One of ['--public-key', '--public-key-file'] must be used".to_string(),
            ))
        }
    }
}

#[derive(Debug, Default, Parser)]
pub struct PrivateKeyInputOptions {
    /// Private key input file name
    #[clap(long, group = "private_key_input", parse(from_os_str))]
    private_key_file: Option<PathBuf>,
    /// Private key encoded in a type as shown in `encoding`
    #[clap(long, group = "private_key_input")]
    private_key: Option<String>,
}

impl PrivateKeyInputOptions {
    pub fn from_private_key(private_key: &Ed25519PrivateKey) -> CliTypedResult<Self> {
        Ok(PrivateKeyInputOptions {
            private_key: Some(
                private_key
                    .to_encoded_string()
                    .map_err(|err| CliError::UnexpectedError(err.to_string()))?,
            ),
            private_key_file: None,
        })
    }

    /// Extract private key from CLI args with fallback to config
    pub fn extract_private_key(
        &self,
        encoding: EncodingType,
        profile: &str,
    ) -> CliTypedResult<Ed25519PrivateKey> {
        if let Some(key) = self.extract_private_key_cli(encoding)? {
            Ok(key)
        } else if let Some(Some(private_key)) =
            CliConfig::load_profile(profile)?.map(|p| p.private_key)
        {
            Ok(private_key)
        } else {
            Err(CliError::CommandArgumentError(
                "One of ['--private-key', '--private-key-file'] must be used".to_string(),
            ))
        }
    }

    /// Extract private key from CLI args
    pub fn extract_private_key_cli(
        &self,
        encoding: EncodingType,
    ) -> CliTypedResult<Option<Ed25519PrivateKey>> {
        if let Some(ref file) = self.private_key_file {
            Ok(Some(
                encoding.load_key("--private-key-file", file.as_path())?,
            ))
        } else if let Some(ref key) = self.private_key {
            let key = key.as_bytes().to_vec();
            Ok(Some(encoding.decode_key("--private-key", key)?))
        } else {
            Ok(None)
        }
    }
}

impl ExtractPublicKey for PrivateKeyInputOptions {
    fn extract_public_key(
        &self,
        encoding: EncodingType,
        profile: &str,
    ) -> CliTypedResult<Ed25519PublicKey> {
        self.extract_private_key(encoding, profile)
            .map(|private_key| private_key.public_key())
    }
}

pub trait ExtractPublicKey {
    fn extract_public_key(
        &self,
        encoding: EncodingType,
        profile: &str,
    ) -> CliTypedResult<Ed25519PublicKey>;

    fn extract_x25519_public_key(
        &self,
        encoding: EncodingType,
        profile: &str,
    ) -> CliTypedResult<x25519::PublicKey> {
        let key = self.extract_public_key(encoding, profile)?;
        x25519::PublicKey::from_ed25519_public_bytes(&key.to_bytes()).map_err(|err| {
            CliError::UnexpectedError(format!("Failed to convert ed25519 to x25519 {:?}", err))
        })
    }
}

pub fn account_address_from_public_key(public_key: &Ed25519PublicKey) -> AccountAddress {
    let auth_key = AuthenticationKey::ed25519(public_key);
    AccountAddress::new(*auth_key.derived_address())
}

/// Options specific to using the Rest endpoint
#[derive(Debug, Default, Parser)]
pub struct RestOptions {
    /// URL to a fullnode on the network
    ///
    /// Defaults to <https://fullnode.devnet.aptoslabs.com>
    #[clap(long, parse(try_from_str))]
    url: Option<Url>,
}

impl RestOptions {
    pub fn new(url: Option<Url>) -> Self {
        RestOptions { url }
    }

    /// Retrieve the URL from the profile or the command line
    pub fn url(&self, profile: &str) -> CliTypedResult<Url> {
        if let Some(ref url) = self.url {
            Ok(url.clone())
        } else if let Some(Some(url)) = CliConfig::load_profile(profile)?.map(|p| p.rest_url) {
            Url::parse(&url).map_err(|err| CliError::UnableToParse("Rest URL", err.to_string()))
        } else {
            Url::parse(DEFAULT_REST_URL).map_err(|err| {
                CliError::UnexpectedError(format!("Failed to parse default rest URL {}", err))
            })
        }
    }

    pub fn client(&self, profile: &str) -> CliTypedResult<Client> {
        Ok(Client::new(self.url(profile)?))
    }
}

/// A wrapper around `AccountAddress` to be more flexible from strings than AccountAddress
#[derive(Clone, Copy, Debug)]
pub struct AccountAddressWrapper {
    pub account_address: AccountAddress,
}

impl FromStr for AccountAddressWrapper {
    type Err = CliError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(AccountAddressWrapper {
            account_address: load_account_arg(s)?,
        })
    }
}

/// Loads an account arg and allows for naming based on profiles
pub fn load_account_arg(str: &str) -> Result<AccountAddress, CliError> {
    if str.starts_with("0x") {
        AccountAddress::from_hex_literal(str).map_err(|err| {
            CliError::CommandArgumentError(format!("Failed to parse AccountAddress {}", err))
        })
    } else if let Ok(account_address) = AccountAddress::from_str(str) {
        Ok(account_address)
    } else if let Some(Some(private_key)) = CliConfig::load_profile(str)?.map(|p| p.private_key) {
        let public_key = private_key.public_key();
        Ok(account_address_from_public_key(&public_key))
    } else {
        Err(CliError::CommandArgumentError(
            "'--account-address' or '--profile' after using aptos init must be provided"
                .to_string(),
        ))
    }
}

/// Options specific to submitting a private key to the Rest endpoint
#[derive(Debug, Parser)]
pub struct WriteTransactionOptions {
    #[clap(flatten)]
    pub private_key_options: PrivateKeyInputOptions,
    #[clap(flatten)]
    pub rest_options: RestOptions,
    /// Maximum gas to be used to publish the package
    ///
    /// Defaults to 1000 gas units
    #[clap(long, default_value_t = DEFAULT_MAX_GAS)]
    pub max_gas: u64,
}

impl Default for WriteTransactionOptions {
    fn default() -> Self {
        Self {
            private_key_options: Default::default(),
            rest_options: Default::default(),
            max_gas: DEFAULT_MAX_GAS,
        }
    }
}

impl WriteTransactionOptions {
    /// Retrieve the chain id from onchain via the Rest API
    pub async fn chain_id(&self, profile: &str) -> CliTypedResult<ChainId> {
        let client = Client::new(self.rest_options.url(profile)?);
        let state = client
            .get_ledger_information()
            .await
            .map_err(|err| CliError::ApiError(err.to_string()))?
            .into_inner();
        Ok(ChainId::new(state.chain_id))
    }
}

pub const DEFAULT_MAX_GAS: u64 = 1000;
pub const DEFAULT_GAS_UNIT_PRICE: u64 = 1;

/// Gas price options for manipulating how to prioritize transactions
#[derive(Debug, Eq, Parser, PartialEq)]
pub struct GasOptions {
    /// Amount to increase gas bid by for a transaction
    ///
    /// Defaults to 1 coin per gas unit
    #[clap(long, default_value_t = DEFAULT_GAS_UNIT_PRICE)]
    pub gas_unit_price: u64,
    /// Maximum gas to be used to send a transaction
    ///
    /// Defaults to 1000 gas units
    #[clap(long, default_value_t = DEFAULT_MAX_GAS)]
    pub max_gas: u64,
}

impl Default for GasOptions {
    fn default() -> Self {
        GasOptions {
            gas_unit_price: DEFAULT_GAS_UNIT_PRICE,
            max_gas: DEFAULT_MAX_GAS,
        }
    }
}

#[derive(Debug, Default, Parser)]
pub struct FaucetOptions {
    /// URL for the faucet
    #[clap(long)]
    faucet_url: Option<Url>,
}

impl FaucetOptions {
    pub fn new(faucet_url: Option<Url>) -> Self {
        FaucetOptions { faucet_url }
    }

    pub fn faucet_url(&self, profile: &str) -> CliTypedResult<Url> {
        if let Some(ref faucet_url) = self.faucet_url {
            Ok(faucet_url.clone())
        } else if let Some(Some(url)) =
            CliConfig::load_profile(profile)?.map(|profile| profile.faucet_url)
        {
            Url::parse(&url)
                .map_err(|err| CliError::UnableToParse("config faucet_url", err.to_string()))
        } else {
            Url::parse(DEFAULT_FAUCET_URL).map_err(|err| {
                CliError::UnexpectedError(format!("Failed to parse default faucet URL {}", err))
            })
        }
    }
}

/// Common options for interacting with an account for a validator
#[derive(Debug, Default, Parser)]
pub struct TransactionOptions {
    #[clap(flatten)]
    pub private_key_options: PrivateKeyInputOptions,
    #[clap(flatten)]
    pub encoding_options: EncodingOptions,
    #[clap(flatten)]
    pub profile_options: ProfileOptions,
    #[clap(flatten)]
    pub rest_options: RestOptions,
    #[clap(flatten)]
    pub gas_options: GasOptions,
}

impl TransactionOptions {
    /// Retrieves the private key
    fn private_key(&self) -> CliTypedResult<Ed25519PrivateKey> {
        self.private_key_options.extract_private_key(
            self.encoding_options.encoding,
            &self.profile_options.profile,
        )
    }

    /// Builds a rest client
    fn rest_client(&self) -> CliTypedResult<Client> {
        self.rest_options.client(&self.profile_options.profile)
    }

    /// Submits a script function based on module name and function inputs
    pub async fn submit_script_function(
        &self,
        address: AccountAddress,
        module: &'static str,
        function: &'static str,
        type_args: Vec<TypeTag>,
        args: Vec<Vec<u8>>,
    ) -> CliTypedResult<Transaction> {
        let txn = TransactionPayload::ScriptFunction(ScriptFunction::new(
            ModuleId::new(address, ident_str!(module).to_owned()),
            ident_str!(function).to_owned(),
            type_args,
            args,
        ));
        self.submit_transaction(txn).await
    }

    /// Submit a transaction
    pub async fn submit_transaction(
        &self,
        payload: TransactionPayload,
    ) -> CliTypedResult<Transaction> {
        let sender_key = self.private_key()?;
        let client = self.rest_client()?;

        // Get sender address
        let sender_address = AuthenticationKey::ed25519(&sender_key.public_key()).derived_address();
        let sender_address = AccountAddress::new(*sender_address);

        // Get sequence number for account
        let sequence_number = get_sequence_number(&client, sender_address).await?;

        // Sign and submit transaction
        let transaction_factory = TransactionFactory::new(get_chain_id(&client).await?)
            .with_gas_unit_price(self.gas_options.gas_unit_price)
            .with_max_gas_amount(self.gas_options.max_gas);
        let sender_account = &mut LocalAccount::new(sender_address, sender_key, sequence_number);
        let transaction =
            sender_account.sign_with_transaction_builder(transaction_factory.payload(payload));
        let response = client
            .submit_and_wait(&transaction)
            .await
            .map_err(|err| CliError::ApiError(err.to_string()))?;

        Ok(response.into_inner())
    }
}

/// Options for compiling a move package dir
#[derive(Debug, Parser)]
pub struct MovePackageDir {
    /// Path to a move package (the folder with a Move.toml file)
    #[clap(long, parse(from_os_str), default_value = ".")]
    pub package_dir: PathBuf,
    /// Path to save the compiled move package
    ///
    /// Defaults to `<package_dir>/build`
    #[clap(long, parse(from_os_str))]
    pub output_dir: Option<PathBuf>,
    /// Named addresses for the move binary
    ///
    /// Example: alice=0x1234, bob=0x5678
    ///
    /// Note: This will fail if there are duplicates in the Move.toml file remove those first.
    #[clap(long, parse(try_from_str = crate::utils::parse_map), default_value = "")]
    pub named_addresses: BTreeMap<String, AccountAddressWrapper>,
}

impl MovePackageDir {
    /// Retrieve the NamedAddresses, resolving all the account addresses accordingly
    pub fn named_addresses(&self) -> BTreeMap<String, AccountAddress> {
        self.named_addresses
            .clone()
            .into_iter()
            .map(|(key, value)| (key, value.account_address))
            .collect()
    }
}
