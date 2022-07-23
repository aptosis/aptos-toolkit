use anyhow::*;
use aptos_upload::AptosUploadTool;
use clitool::CliTool;

#[tokio::main]
async fn main() -> Result<()> {
    AptosUploadTool::execute_main().await
}
