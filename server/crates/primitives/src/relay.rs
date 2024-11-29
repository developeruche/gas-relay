use alloy::primitives::{B256, U256};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum RequestState {
    Pending, // This means the request has been sent but not yet mined
    Success, // This means the request has been mined and successful
    Failed,  // This means the request has been mined and failed
    Timeout, // This request took too long to be mined
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RelayRequest {
    from: String,
    to: String,
    value: u64,
    gas: u64,
    deadline: u64,
    data: String,
    nonce: u64,
    signature: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RequestStatus {
    pub chain_id: u64,
    pub request_id: String,
    pub request_state: RequestState,
    pub created_at: NaiveDateTime,
    pub transaction_hash: String,
    pub block_number: u64,
    pub mined_at: NaiveDateTime,
    pub gas_used: u64,
}

// impl to convert a string to RequestState
impl From<String> for RequestState {
    fn from(s: String) -> Self {
        let s = s.as_str();
        match s {
            "Pending" => Self::Pending,
            "Success" => Self::Success,
            "Failed" => Self::Failed,
            "Timeout" => Self::Timeout,
            _ => panic!("Invalid request state"),
        }
    }
}

// impl to convert a RequestState to a string
impl From<RequestState> for String {
    fn from(s: RequestState) -> Self {
        match s {
            RequestState::Pending => "Pending".to_string(),
            RequestState::Success => "Success".to_string(),
            RequestState::Failed => "Failed".to_string(),
            RequestState::Timeout => "Timeout".to_string(),
        }
    }
}

pub fn generate_request_id() -> String {
    B256::random().to_string()
}