use std::error::Error;
use std::str::FromStr;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Transaction {
    #[serde(rename = "tx")]
    pub id: u32,
    #[serde(rename = "type")]
    pub kind: String,
    pub amount: Option<f32>,
    #[serde(rename = "client")]
    pub client_id: u16,
}

pub enum TransactionType {
    Deposit,
    Withdrawal,
    Dispute,
    Resolve,
    Chargeback,
}

impl FromStr for TransactionType {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "deposit" => Ok(TransactionType::Deposit),
            "withdrawal" => Ok(TransactionType::Withdrawal),
            "dispute" => Ok(TransactionType::Dispute),
            "resolve" => Ok(TransactionType::Resolve),
            "chargeback" => Ok(TransactionType::Chargeback),
            _ => Err("Invalid transaction type".into()),
        }
    }
}
