use anyhow::*;
use aptos_create_resource_account::AptosCreateResourceAccountTool;
use clitool::CliTool;

#[tokio::main]
async fn main() -> Result<()> {
    AptosCreateResourceAccountTool::execute_main().await
}
