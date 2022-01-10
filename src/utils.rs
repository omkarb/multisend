use crate::{Error, Result};
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub struct TransferInstruction {
    pub address: String,
    pub amount: f64,
    pub coin: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub struct MultisendInstruction {
    pub recipients: Vec<TransferInstruction>,
    pub senders: Vec<TransferInstruction>,
}

pub fn read_instruction_json(config: &str) -> Result<MultisendInstruction> {
    let data = fs::read_to_string(config)?;
    let json: MultisendInstruction =
        serde_json::from_str(&data).expect("JSON was not well-formatted");
    Ok(json)
}

pub fn validate_tx_amounts(data: &MultisendInstruction) -> Result<bool> {
    let recipient_amount = data.recipients.iter().fold(0.0, |acc, x| acc + x.amount);
    println!("Recipient amount {}", recipient_amount);
    let senders_amount = data.senders.iter().fold(0.0, |acc, x| acc + x.amount);
    println!("Sender amount {}", senders_amount);
    if (recipient_amount - senders_amount).abs() > 0.01 {
        return Err(Error::InvalidConfig(
            "Sender & Receiver amount mismatch".to_string(),
        ));
    }
    Ok(true)
}
