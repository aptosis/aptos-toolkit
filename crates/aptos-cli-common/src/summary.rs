use aptos_rest_client::{aptos_api_types::WriteSetChange, Transaction};
use aptos_types::account_address::AccountAddress;
use serde::Serialize;

/// A shortened transaction output
#[derive(Clone, Debug, Default, Serialize)]
pub struct TransactionSummary {
    changes: Vec<ChangeSummary>,
    gas_used: Option<u64>,
    success: bool,
    version: Option<u64>,
    vm_status: String,
}

impl From<Transaction> for TransactionSummary {
    fn from(transaction: Transaction) -> Self {
        let mut summary = TransactionSummary {
            success: transaction.success(),
            version: transaction.version(),
            vm_status: transaction.vm_status(),
            ..Default::default()
        };

        if let Ok(info) = transaction.transaction_info() {
            summary.gas_used = Some(info.gas_used.0);
            summary.changes = info
                .changes
                .iter()
                .map(|change| match change {
                    WriteSetChange::DeleteModule { module, .. } => ChangeSummary {
                        event: change.type_str(),
                        module: Some(module.to_string()),
                        ..Default::default()
                    },
                    WriteSetChange::DeleteResource {
                        address, resource, ..
                    } => ChangeSummary {
                        event: change.type_str(),
                        address: Some(*address.inner()),
                        resource: Some(resource.to_string()),
                        ..Default::default()
                    },
                    WriteSetChange::DeleteTableItem { handle, key, .. } => ChangeSummary {
                        event: change.type_str(),
                        handle: Some(handle.to_string()),
                        key: Some(key.to_string()),
                        ..Default::default()
                    },
                    WriteSetChange::WriteModule { address, .. } => ChangeSummary {
                        event: change.type_str(),
                        address: Some(*address.inner()),
                        ..Default::default()
                    },
                    WriteSetChange::WriteResource { address, data, .. } => ChangeSummary {
                        event: change.type_str(),
                        address: Some(*address.inner()),
                        resource: Some(data.typ.to_string()),
                        data: Some(serde_json::to_value(&data.data).unwrap_or_default()),
                        ..Default::default()
                    },
                    WriteSetChange::WriteTableItem {
                        handle, key, value, ..
                    } => ChangeSummary {
                        event: change.type_str(),
                        handle: Some(handle.to_string()),
                        key: Some(key.to_string()),
                        value: Some(value.to_string()),
                        ..Default::default()
                    },
                })
                .collect();
        }

        summary
    }
}

/// A summary of a [`WriteSetChange`] for easy printing
#[derive(Clone, Debug, Default, Serialize)]
pub struct ChangeSummary {
    #[serde(skip_serializing_if = "Option::is_none")]
    address: Option<AccountAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<serde_json::Value>,
    event: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    handle: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    module: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<String>,
}
