//! CLI for creating Aptos resource accounts.
//!
//! *Note: this CLI will be changed in the near future, since resource accounts are still being developed in aptos-core.*
//!
//! # Setup
//!
//! To use this CLI, you may install this crate via Cargo:
//!
//! ```bash
//! cargo install aptos-create-resource-account
//! ```
//!
//! You should also have an Aptos CLI configuration file, which can be created using:
//!
//! ```bash
//! aptos init
//! ```
//!
//! # Usage
//!
//! Run the following command to create a new resource account:
//!
//! ```bash
//! aptos-create-resource-account my-account
//! ```
//!
//! This will create a profile in your `.aptos/config.yaml` with the name `my-account`.
//! You can then use this profile to publish modules, run scripts, etc.
use anyhow::Result;
use aptos_cli_common::{
    utils::fund_account, FaucetOptions, TransactionOptions, TransactionSummary,
};
use aptos_cli_config::{CliConfig, ProfileConfig};
use aptos_crypto::ed25519::Ed25519PublicKey;
use aptos_keygen::KeyGen;
use aptos_rest_client::{aptos_api_types::WriteSetChange, Transaction};
use aptos_types::{
    account_address::AccountAddress,
    transaction::{authenticator::AuthenticationKey, ScriptFunction, TransactionPayload},
};
use async_trait::async_trait;
use clap::Parser;
use clitool::CliTool;
use move_core_types::{ident_str, language_storage::ModuleId};
use serde::{Deserialize, Serialize};

pub const DEFAULT_FUNDED_COINS: u64 = 10000;

/// Module address of the deployer.
pub const APTOS_FRAMEWORK_ADDRESS: AccountAddress = static_address::static_address!("0x1");

/// Command to create a new account on-chain
///
#[derive(Debug, Parser)]
#[clap(name = "aptos-create-resource-account", about, version)]
pub struct AptosCreateResourceAccountTool {
    #[clap(flatten)]
    pub transaction_options: TransactionOptions,
    /// Flag for using faucet to create the account
    #[clap(long)]
    pub use_faucet: bool,
    #[clap(flatten)]
    pub faucet_options: FaucetOptions,
    /// Initial coins to fund when using the faucet
    #[clap(long, default_value_t = DEFAULT_FUNDED_COINS)]
    pub initial_coins: u64,
    /// Name of the profile to create. This is also the seed.
    pub new_profile: String,
}

#[async_trait]
impl CliTool<AptosCreateResourceAccountResult> for AptosCreateResourceAccountTool {
    async fn execute(self) -> Result<AptosCreateResourceAccountResult> {
        let result = self.generate_profile().await?;

        if self.use_faucet {
            fund_account(
                self.faucet_options.faucet_url(&result.profile.name)?,
                self.initial_coins,
                &result.profile.account,
            )
            .await?;
        }

        Ok(result)
    }
}

/// An individual profile
#[derive(Clone, Debug, Serialize)]
pub struct AptosCreateResourceAccountResult {
    pub profile: ProfileInfo,
    pub tx: TransactionSummary,
}

/// An individual profile
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProfileInfo {
    /// Profile name.
    pub name: String,
    /// Public key for commands
    pub public_key: Ed25519PublicKey,
    /// Account created.
    pub account: AccountAddress,
}

impl AptosCreateResourceAccountTool {
    /// Creates the resource account using the provided public key.
    pub async fn create_resource_account_with_public_key(
        &self,
        public_key: &Ed25519PublicKey,
    ) -> aptos_cli_config::CliTypedResult<Transaction> {
        let auth_key = AuthenticationKey::ed25519(public_key);
        let create_account_fn = TransactionPayload::ScriptFunction(ScriptFunction::new(
            ModuleId::new(
                APTOS_FRAMEWORK_ADDRESS,
                ident_str!("ResourceAccount").to_owned(),
            ),
            ident_str!("create_resource_account").to_owned(),
            vec![],
            vec![
                bcs::to_bytes(self.new_profile.as_bytes())?,
                bcs::to_bytes(&auth_key)?,
            ],
        ));
        self.transaction_options
            .submit_transaction(create_account_fn)
            .await
    }

    async fn generate_profile(&self) -> Result<AptosCreateResourceAccountResult> {
        // First we must generate a new keypair. This will be the keypair of the created account.
        let (module_private_key, module_auth_key) =
            KeyGen::from_os_rng().generate_ed25519_keypair();

        // Next we'll create the account as a resource account.
        let create_tx = self
            .create_resource_account_with_public_key(&module_auth_key)
            .await?;

        let module_account_key = create_tx
            .transaction_info()?
            .changes
            .iter()
            .find_map(|change| match change {
                WriteSetChange::WriteResource {
                    address,
                    state_key_hash: _,
                    data,
                } => {
                    if data.typ.name.as_str() == "SignerCapabilityStore" {
                        Some(address.inner())
                    } else {
                        None
                    }
                }
                _ => None,
            })
            .unwrap();

        let rest_url = self
            .transaction_options
            .rest_options
            .url(&self.transaction_options.profile_options.profile)?;

        let new_profile = ProfileConfig {
            private_key: Some(module_private_key),
            public_key: Some(module_auth_key.clone()),
            account: Some(*module_account_key),
            rest_url: Some(rest_url.to_string()),
            faucet_url: Some(
                self.faucet_options
                    .faucet_url(&self.transaction_options.profile_options.profile)?
                    .to_string(),
            ),
        };

        let config: &mut CliConfig = &mut CliConfig::load()?;
        config
            .profiles
            .as_mut()
            .unwrap()
            .insert(self.new_profile.clone(), new_profile);
        config.save()?;

        let profile_info = ProfileInfo {
            name: self.new_profile.clone(),
            public_key: module_auth_key.clone(),
            account: *module_account_key,
        };
        Ok(AptosCreateResourceAccountResult {
            profile: profile_info,
            tx: create_tx.into(),
        })
    }
}
