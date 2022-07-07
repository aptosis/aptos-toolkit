//! CLI for publishing Move modules to Aptos.
//!
//! # Setup
//!
//! To use this CLI, you may install this crate via Cargo:
//!
//! ```bash
//! cargo install aptos-publish
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
//! Run the following command to publish the module in the current directory:
//!
//! ```bash
//! aptos-publish
//! ```
use std::path::Path;

use anyhow::Result;
use aptos_cli_common::{
    utils::get_sequence_number, EncodingOptions, MovePackageDir, ProfileOptions,
    TransactionSummary, WriteTransactionOptions,
};
use aptos_cli_config::{CliError, CliTypedResult};
use aptos_crypto::ed25519::Ed25519PrivateKey;
use aptos_rest_client::{Client, Transaction};
use aptos_sdk::{transaction_builder::TransactionFactory, types::LocalAccount};
use aptos_types::{
    account_address::AccountAddress,
    chain_id::ChainId,
    transaction::{ModuleBundle, TransactionPayload},
};
use async_trait::async_trait;
use clap::Parser;
use clitool::CliTool;
use move_deps::{
    move_command_line_common::env::get_bytecode_version_from_env,
    move_package::{compilation::compiled_package::CompiledPackage, BuildConfig},
};
use url::Url;

/// Command to publish a Move module to Aptos
#[derive(Debug, Parser)]
#[clap(name = "aptos-publish", about, version)]
pub struct AptosPublishTool {
    #[clap(flatten)]
    pub encoding_options: EncodingOptions,
    #[clap(flatten)]
    pub move_options: MovePackageDir,
    #[clap(flatten)]
    pub write_options: WriteTransactionOptions,
    #[clap(flatten)]
    pub profile_options: ProfileOptions,
}

/// Compiles a Move package dir, and returns the compiled modules.
fn compile_move(build_config: BuildConfig, package_dir: &Path) -> CliTypedResult<CompiledPackage> {
    // TODO: Add caching
    build_config
        .compile_package(package_dir, &mut Vec::new())
        .map_err(|err| CliError::MoveCompilationError(err.to_string()))
}

#[async_trait]
impl CliTool<TransactionSummary> for AptosPublishTool {
    async fn execute(self) -> Result<TransactionSummary> {
        let build_config = BuildConfig {
            additional_named_addresses: self.move_options.named_addresses(),
            generate_abis: false,
            generate_docs: true,
            install_dir: self.move_options.output_dir.clone(),
            ..Default::default()
        };
        let package = compile_move(build_config, self.move_options.package_dir.as_path())?;
        let compiled_units: Vec<Vec<u8>> = package
            .root_compiled_units
            .iter()
            .map(|unit_with_source| {
                unit_with_source
                    .unit
                    .serialize(get_bytecode_version_from_env())
            })
            .collect();
        let compiled_payload = TransactionPayload::ModuleBundle(ModuleBundle::new(compiled_units));

        // Now that it's compiled, lets send it
        let sender_key = self.write_options.private_key_options.extract_private_key(
            self.encoding_options.encoding,
            &self.profile_options.profile,
        )?;

        submit_transaction(
            self.write_options
                .rest_options
                .url(&self.profile_options.profile)?,
            self.write_options
                .chain_id(&self.profile_options.profile)
                .await?,
            self.profile_options.account_address()?,
            sender_key,
            compiled_payload,
            self.write_options.max_gas,
        )
        .await
        .map(TransactionSummary::from)
        .map_err(|err| anyhow::anyhow!("failed to submit transaction: {}", err))
    }
}

/// Submits a [`TransactionPayload`] as signed by the `sender_key`
pub async fn submit_transaction(
    url: Url,
    chain_id: ChainId,
    sender_address: AccountAddress,
    sender_key: Ed25519PrivateKey,
    payload: TransactionPayload,
    max_gas: u64,
) -> CliTypedResult<Transaction> {
    let client = Client::new(url);

    // Get sequence number for account
    let sequence_number = get_sequence_number(&client, sender_address).await?;

    // Sign and submit transaction
    let transaction_factory = TransactionFactory::new(chain_id)
        .with_gas_unit_price(1)
        .with_max_gas_amount(max_gas);
    let sender_account = &mut LocalAccount::new(sender_address, sender_key, sequence_number);
    let transaction =
        sender_account.sign_with_transaction_builder(transaction_factory.payload(payload));
    let response = client
        .submit_and_wait(&transaction)
        .await
        .map_err(|err| CliError::ApiError(err.to_string()))?;

    Ok(response.into_inner())
}
