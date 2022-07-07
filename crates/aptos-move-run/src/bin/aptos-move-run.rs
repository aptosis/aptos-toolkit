use anyhow::*;
use aptos_move_run::AptosMoveRunTool;
use clitool::CliTool;

#[tokio::main]
async fn main() -> Result<()> {
    AptosMoveRunTool::execute_main().await
}
