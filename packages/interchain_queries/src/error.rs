use cosmwasm_std::StdError;
use protobuf::Error as ProtobufError;
use schemars::_serde_json;
use thiserror::Error;

pub type ContractResult<T> = Result<T, ContractError>;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Bech32 error")]
    Bech32(#[from] bech32::Error),

    #[error("Prost protobuf error")]
    ProstProtobuf(#[from] prost::DecodeError),

    #[error("Protobuf error")]
    Protobuf(String),

    #[error("balance with denom '{denom:?}' for address '{recipient:?}' not found")]
    BalanceNotFound { denom: String, recipient: String },

    #[error("empty stargate result for query type '{query_type:?}'")]
    EmptyStargateResult { query_type: String },

    #[error("interchain query for {zone_id:?} {query_type:?} {query_data_json_encoded:?} is not registered")]
    InterchainQueryIsNotRegistered {
        zone_id: String,
        query_type: String,
        query_data_json_encoded: String,
    },

    #[error("address length should be max {max:?} bytes, got {actual:?}")]
    MaxAddrLength { max: usize, actual: usize },

    #[error("no result data in register interchain query response")]
    EmptyInterchainQueryResult,

    #[error("register interchain query failed: {0}")]
    RegisterInterchainQueryFailed(String),

    #[error("invalid reply id: {0}")]
    InvalidReplyID(u64),
}

impl From<ProtobufError> for ContractError {
    fn from(e: ProtobufError) -> Self {
        ContractError::Protobuf(e.to_string())
    }
}

impl From<_serde_json::Error> for ContractError {
    fn from(e: _serde_json::Error) -> Self {
        ContractError::Protobuf(e.to_string())
    }
}
