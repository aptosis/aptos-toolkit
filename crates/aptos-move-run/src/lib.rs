//! CLI for publishing Move modules to Aptos.
//!
//! # Setup
//!
//! To use this CLI, you may install this crate via Cargo:
//!
//! ```bash
//! cargo install aptos-move-run
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
//! aptos-move-run
//! ```
use anyhow::Result;
use aptos_cli_common::{load_account_arg, TransactionOptions, TransactionSummary};
use aptos_cli_config::{CliError, CliTypedResult};
use aptos_rest_client::aptos_api_types::MoveType;
use aptos_types::{
    account_address::AccountAddress,
    transaction::{ScriptFunction, TransactionPayload},
};
use async_trait::async_trait;
use clap::Parser;
use clitool::CliTool;
use move_core_types::{
    identifier::Identifier,
    language_storage::{ModuleId, TypeTag},
};
use std::str::FromStr;

#[derive(Clone, Debug)]
enum FunctionArgType {
    Address,
    Bool,
    Hex,
    String,
    U8,
    U64,
    U128,
}

impl FunctionArgType {
    fn parse_arg(&self, arg: &str) -> CliTypedResult<Vec<u8>> {
        match self {
            FunctionArgType::Address => bcs::to_bytes(
                &AccountAddress::from_str(arg)
                    .map_err(|err| CliError::UnableToParse("address", err.to_string()))?,
            ),
            FunctionArgType::Bool => bcs::to_bytes(
                &bool::from_str(arg)
                    .map_err(|err| CliError::UnableToParse("bool", err.to_string()))?,
            ),
            FunctionArgType::Hex => bcs::to_bytes(
                &hex::decode(arg).map_err(|err| CliError::UnableToParse("hex", err.to_string()))?,
            ),
            FunctionArgType::String => bcs::to_bytes(arg),
            FunctionArgType::U8 => bcs::to_bytes(
                &u8::from_str(arg).map_err(|err| CliError::UnableToParse("u8", err.to_string()))?,
            ),
            FunctionArgType::U64 => bcs::to_bytes(
                &u64::from_str(arg)
                    .map_err(|err| CliError::UnableToParse("u64", err.to_string()))?,
            ),
            FunctionArgType::U128 => bcs::to_bytes(
                &u128::from_str(arg)
                    .map_err(|err| CliError::UnableToParse("u128", err.to_string()))?,
            ),
        }
        .map_err(|err| CliError::BCS("arg", err))
    }
}

impl FromStr for FunctionArgType {
    type Err = CliError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "address" => Ok(FunctionArgType::Address),
            "bool" => Ok(FunctionArgType::Bool),
            "hex" => Ok(FunctionArgType::Hex),
            "string" => Ok(FunctionArgType::String),
            "u8" => Ok(FunctionArgType::U8),
            "u64" => Ok(FunctionArgType::U64),
            "u128" => Ok(FunctionArgType::U128),
            str => Err(CliError::CommandArgumentError(format!("Invalid arg type '{}'.  Must be one of: ['address','bool','hex','string','u8','u64','u128']", str))),
        }
    }
}

/// A parseable arg with a type separated by a colon
#[derive(Clone, Debug)]
pub struct ArgWithType {
    _ty: FunctionArgType,
    arg: Vec<u8>,
}

impl FromStr for ArgWithType {
    type Err = CliError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split(':').collect();
        if parts.len() != 2 {
            return Err(CliError::CommandArgumentError(
                "Arguments must be pairs of <type>:<arg> e.g. bool:true".to_string(),
            ));
        }

        let ty = FunctionArgType::from_str(parts.first().unwrap())?;
        let arg = parts.last().unwrap();
        let arg = ty.parse_arg(arg)?;

        Ok(ArgWithType { _ty: ty, arg })
    }
}

#[derive(Debug)]
pub struct FunctionId {
    pub module_id: ModuleId,
    pub function_id: Identifier,
}

fn parse_function_name(function_id: &str) -> CliTypedResult<FunctionId> {
    let ids: Vec<&str> = function_id.split_terminator("::").collect();
    if ids.len() != 3 {
        return Err(CliError::CommandArgumentError(
            "FunctionId is not well formed.  Must be of the form <address>::<module>::<function>"
                .to_string(),
        ));
    }
    let address = load_account_arg(ids.get(0).unwrap())?;
    let module = Identifier::from_str(ids.get(1).unwrap())
        .map_err(|err| CliError::UnableToParse("Module Name", err.to_string()))?;
    let function_id = Identifier::from_str(ids.get(2).unwrap())
        .map_err(|err| CliError::UnableToParse("Function Name", err.to_string()))?;
    let module_id = ModuleId::new(address, module);
    Ok(FunctionId {
        module_id,
        function_id,
    })
}

/// Run an arbitrary Move command.
#[derive(Debug, Parser)]
#[clap(name = "aptos-move-run", about, version)]
pub struct AptosMoveRunTool {
    #[clap(flatten)]
    pub txn_options: TransactionOptions,
    /// Function name as `<ADDRESS>::<MODULE_ID>::<FUNCTION_NAME>`
    ///
    /// Example: `0x842ed41fad9640a2ad08fdd7d3e4f7f505319aac7d67e1c0dd6a7cce8732c7e3::Message::set_message`
    #[clap(long, parse(try_from_str = parse_function_name))]
    pub function_id: FunctionId,
    /// Hex encoded arguments separated by spaces.
    ///
    /// Example: `0x01 0x02 0x03`
    #[clap(long, multiple_values = true)]
    pub args: Vec<ArgWithType>,
    /// TypeTag arguments separated by spaces.
    ///
    /// Example: `u8 u64 u128 bool address vector true false signer`
    #[clap(long, multiple_values = true)]
    pub type_args: Vec<MoveType>,
}

#[async_trait]
impl CliTool<TransactionSummary> for AptosMoveRunTool {
    async fn execute(self) -> Result<TransactionSummary> {
        let args: Vec<Vec<u8>> = self
            .args
            .iter()
            .map(|arg_with_type| arg_with_type.arg.clone())
            .collect();
        let mut type_args: Vec<TypeTag> = Vec::new();

        // These TypeArgs are used for generics
        for type_arg in self.type_args.iter().cloned() {
            let type_tag = TypeTag::try_from(type_arg)
                .map_err(|err| CliError::UnableToParse("--type-args", err.to_string()))?;
            type_args.push(type_tag)
        }

        self.txn_options
            .submit_transaction(TransactionPayload::ScriptFunction(ScriptFunction::new(
                self.function_id.module_id.clone(),
                self.function_id.function_id.clone(),
                type_args,
                args,
            )))
            .await
            .map(TransactionSummary::from)
            .map_err(|err| anyhow::anyhow!("failed to submit transaction: {}", err))
    }
}
