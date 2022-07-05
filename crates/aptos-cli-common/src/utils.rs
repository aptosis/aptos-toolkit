use aptos_cli_config::error::{CliError, CliTypedResult};
use aptos_rest_client::Client;
use aptos_types::{account_address::AccountAddress, chain_id::ChainId};
use reqwest::Url;

/// Retrieves sequence number from the rest client
pub async fn get_sequence_number(
    client: &aptos_rest_client::Client,
    address: AccountAddress,
) -> CliTypedResult<u64> {
    let account_response = client
        .get_account(address)
        .await
        .map_err(|err| CliError::ApiError(err.to_string()))?;
    let account = account_response.inner();
    Ok(account.sequence_number)
}

/// Retrieves the chain id from the rest client
pub async fn get_chain_id(rest_client: &Client) -> CliTypedResult<ChainId> {
    let state = rest_client
        .get_ledger_information()
        .await
        .map_err(|err| CliError::ApiError(err.to_string()))?
        .into_inner();
    Ok(ChainId::new(state.chain_id))
}

/// Fund account (and possibly create it) from a faucet
pub async fn fund_account(
    faucet_url: Url,
    num_coins: u64,
    address: &AccountAddress,
) -> CliTypedResult<()> {
    let response = reqwest::Client::new()
        .post(format!(
            "{}mint?amount={}&auth_key={}",
            faucet_url, num_coins, address
        ))
        .send()
        .await
        .map_err(|err| CliError::ApiError(err.to_string()))?;
    if response.status() == 200 {
        Ok(())
    } else {
        Err(CliError::ApiError(format!(
            "Faucet issue: {}",
            response.status()
        )))
    }
}
