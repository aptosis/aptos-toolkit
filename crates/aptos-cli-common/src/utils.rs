use std::{collections::BTreeMap, str::FromStr};

use aptos_cli_config::error::{CliError, CliTypedResult};
use aptos_rest_client::Client;
use aptos_types::{account_address::AccountAddress, chain_id::ChainId};
use itertools::Itertools;
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

/// Error message for parsing a map
const PARSE_MAP_SYNTAX_MSG: &str = "Invalid syntax for map. Example: Name=Value,Name2=Value";

/// Parses an inline map of values
///
/// Example: Name=Value,Name2=Value
pub fn parse_map<K: FromStr + Ord, V: FromStr>(str: &str) -> anyhow::Result<BTreeMap<K, V>>
where
    K::Err: 'static + std::error::Error + Send + Sync,
    V::Err: 'static + std::error::Error + Send + Sync,
{
    let mut map = BTreeMap::new();

    // Split pairs by commas
    for pair in str.split_terminator(',') {
        // Split pairs by = then trim off any spacing
        let (first, second): (&str, &str) = pair
            .split_terminator('=')
            .collect_tuple()
            .ok_or_else(|| anyhow::Error::msg(PARSE_MAP_SYNTAX_MSG))?;
        let first = first.trim();
        let second = second.trim();
        if first.is_empty() || second.is_empty() {
            return Err(anyhow::Error::msg(PARSE_MAP_SYNTAX_MSG));
        }

        // At this point, we just give error messages appropriate to parsing
        let key: K = K::from_str(first)?;
        let value: V = V::from_str(second)?;
        map.insert(key, value);
    }
    Ok(map)
}
