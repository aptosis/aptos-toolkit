//! CLI for publishing already-compiled Move modules to Aptos.
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

use anyhow::Result;
use aptos_cli_common::{MovePackageDir, TransactionOptions, TransactionSummary};
use async_trait::async_trait;
use clap::Parser;
use clitool::CliTool;
use move_package::compilation::compiled_package::{CompiledPackage, OnDiskCompiledPackage};

/// Command to publish a Move module to Aptos
#[derive(Debug, Parser)]
#[clap(name = "aptos-upload", about, version)]
pub struct AptosUploadTool {
    #[clap(flatten)]
    pub move_options: MovePackageDir,
    #[clap(flatten)]
    pub txn_options: TransactionOptions,
    /// Name of the package.
    pub package_name: String,
}

#[async_trait]
impl CliTool<TransactionSummary> for AptosUploadTool {
    async fn execute(self) -> Result<TransactionSummary> {
        let package: CompiledPackage = OnDiskCompiledPackage::from_path(
            &self
                .move_options
                .output_dir
                .unwrap_or_else(|| self.move_options.package_dir.join("build"))
                .join(&self.package_name),
        )?
        .into_compiled_package()?;
        aptos_publish::publish(&package, &self.txn_options).await
    }
}
