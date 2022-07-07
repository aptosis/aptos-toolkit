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
use aptos_cli_common::{MovePackageDir, TransactionOptions, TransactionSummary};
use aptos_cli_config::{CliError, CliTypedResult};
use aptos_types::transaction::{ModuleBundle, TransactionPayload};
use async_trait::async_trait;
use clap::Parser;
use clitool::CliTool;
use move_deps::{
    move_command_line_common::env::get_bytecode_version_from_env,
    move_package::{compilation::compiled_package::CompiledPackage, BuildConfig},
};

/// Command to publish a Move module to Aptos
#[derive(Debug, Parser)]
#[clap(name = "aptos-publish", about, version)]
pub struct AptosPublishTool {
    #[clap(flatten)]
    pub move_options: MovePackageDir,
    #[clap(flatten)]
    pub txn_options: TransactionOptions,
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

        // Send the compiled module
        self.txn_options
            .submit_transaction(TransactionPayload::ModuleBundle(ModuleBundle::new(
                compiled_units,
            )))
            .await
            .map(TransactionSummary::from)
            .map_err(|err| anyhow::anyhow!("failed to submit transaction: {}", err.to_string()))
    }
}
