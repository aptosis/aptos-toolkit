use anyhow::*;
use aptos_publish::AptosPublishTool;
use clitool::CliTool;

#[tokio::main]
async fn main() -> Result<()> {
    AptosPublishTool::execute_main().await
}
