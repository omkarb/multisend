use crate::{utils, Error, Result};
use core::str::FromStr;
use solana_clap_utils::keypair;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    derivation_path::DerivationPath,
    instruction::Instruction,
    message::Message,
    pubkey::Pubkey,
    signer::{keypair::Keypair, Signer},
    system_instruction,
    transaction::Transaction,
};
const LAMPORTS_PER_SOL: f64 = 1_000_000_000.0;

pub fn initialize_wallet(
    keypair_name: &str,
    mut derivation_path: Option<DerivationPath>,
) -> Result<Keypair> {
    if derivation_path == None {
        derivation_path =
        // Using default derivation path for now. TODO: Add config option.
            Some(DerivationPath::from_key_str("0/0").expect("Invalid derivation path"))
    }
    let keys =
        keypair::keypair_from_seed_phrase(keypair_name, false, false, derivation_path, false)
            .expect("Couldn't derive seed");
    Ok(keys)
}

fn rpc_connection(network: &str) -> RpcClient {
    let url = match network {
        // Config option.
        "devnet" => String::from("https://api.devnet.solana.com"),
        "mainnet" => String::from("https://api.mainnet-beta.solana.com"),
        _ => String::from("https://api.devnet.solana.com"),
    };
    RpcClient::new(url)
}

fn get_balance(client: &RpcClient, keypair: &Keypair) -> Result<u64> {
    let balance = client.get_balance(&keypair.pubkey())?;
    Ok(balance)
}

pub fn send_transaction(
    keypair: &Keypair,
    network: &str,
    instructions: Vec<Instruction>,
) -> Result<()> {
    // Initialize connection to solana rpc node
    let client = rpc_connection(network);
    // build message out of the set of the instructions passed in via the params.
    let message = Message::new(&instructions, Some(&keypair.pubkey()));
    // Build transaction object. We need to most recent block hash to prove to the network that we're
    // using a node thath has observed a recent block (is in sync w/ the rest of the network)
    let transaction = Transaction::new(&[keypair], message, client.get_recent_blockhash()?.0);
    // sign and send the transction
    let _signature = client.send_and_confirm_transaction_with_spinner(&transaction)?;
    Ok(())
}

pub fn build_transfer_instruction(
    keypair: &Keypair,
    data: &utils::MultisendInstruction,
) -> Vec<Instruction> {
    let recipients: Vec<(Pubkey, u64)> = data
        .recipients
        .iter()
        .map(|instr| {
            (
                Pubkey::from_str(&instr.address).unwrap(),
                (instr.amount * LAMPORTS_PER_SOL) as u64,
            )
        })
        .collect();
    system_instruction::transfer_many(&keypair.pubkey(), &recipients)
}

pub fn validate_addrs(data: &utils::MultisendInstruction) -> Result<()> {
    for instr in &data.recipients {
        let _status = match Pubkey::from_str(&instr.address) {
            Ok(_s) => _s,
            Err(_) => {
                return Err(Error::InvalidConfig(format!(
                    "Address {} not a valid address",
                    instr.address
                )))
            }
        };
    }

    for instr in &data.senders {
        let _status = match Pubkey::from_str(&instr.address) {
            Ok(_s) => _s,
            Err(_) => {
                return Err(Error::InvalidConfig(format!(
                    "Address {} not a valid address",
                    instr.address
                )))
            }
        };
    }
    Ok(())
}

pub fn validate_balance(network: &str, data: &utils::MultisendInstruction) -> Result<()> {
    let keypair = initialize_wallet("wallet", None).unwrap();
    let client = rpc_connection(network);
    for sender in &data.senders {
        let balance = get_balance(&client, &keypair)?;
        println!(
            "Balance for {} is {}",
            sender.address,
            balance as f64 / LAMPORTS_PER_SOL
        );
        if balance < (sender.amount * LAMPORTS_PER_SOL) as u64 {
            return Err(Error::InvalidConfig(format!(
                "Address {} has a lower balance than send amount.",
                sender.address
            )));
        }
    }

    Ok(())
}
