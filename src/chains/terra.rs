extern crate base64;
extern crate rpassword;
use crate::{utils, Error, Result};
use rpassword::read_password;
use secp256k1::Secp256k1;
use serde::{Deserialize, Serialize};
use std::io::Write;
use tokio;

const TERRA_DECIMAL: f64 = 1_000_000.0;
use terra_rust_api::core_types::Coin;
use terra_rust_api::{GasOptions, Message, MsgExecuteContract, PrivateKey, PublicKey, Terra};

#[derive(Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg<'a> {
    Transfer { amount: &'a str, recipient: &'a str },
}

impl ExecuteMsg<'static> {
    pub fn create_transfer<'a>(
        amount: u64,
        from_address: &'a str,
        to_address: &'a str,
        contract: &'a str,
    ) -> Message {
        let transfer = ExecuteMsg::Transfer {
            amount: &amount.to_string(),
            recipient: to_address,
        };
        let tranfser_json = serde_json::to_string(&transfer).unwrap();
        let coins: Vec<Coin> = vec![];
        MsgExecuteContract::create_from_json(from_address, contract, &tranfser_json, &coins)
            .unwrap()
    }
}

#[tokio::main]
pub async fn send_transaction(
    network: &str,
    gas_price: &str,
    gas_adj: f64,
    memo: &String,
    from_key: PrivateKey,
    msgs: Vec<Message>,
) -> Result<()> {
    let client = rpc_connection(network, gas_price, gas_adj).unwrap();
    let secp = Secp256k1::new();
    print!("{}, {}, {:?}", gas_price, gas_adj, memo.to_string());
    // let resp = client
    //     .submit_transaction_sync(&secp, &from_key, msgs, Some(memo.to_string()))
    //     .await
    //     .unwrap();
    // let hash = resp.txhash;
    // print!("https://finder.terra.money/mainnet/tx/{}", hash);
    Ok(())
}

pub fn build_transfer_msgs(
    sender_address: &PublicKey,
    data: &utils::MultisendInstruction,
) -> Vec<Message> {
    let transfer: Vec<Message> = data
        .recipients
        .iter()
        .map(|instr| {
            ExecuteMsg::create_transfer(
                (instr.amount * TERRA_DECIMAL) as u64,
                &sender_address.account().unwrap(),
                &instr.address,
                &instr.coin,
            )
        })
        .collect();
    return transfer;
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct QueryResult {
    balance: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Balance { address: String },

    QueryResult(QueryResult),
}

impl QueryMsg {
    fn base64encode(&self) -> String {
        let serialized = serde_json::to_string(&self).unwrap();
        base64::encode(serialized)
    }

    fn balance(self) -> u64 {
        if let QueryMsg::QueryResult(result) = self {
            result.balance.parse::<u64>().unwrap()
        } else {
            panic!("Not a query result")
        }
    }
}

pub fn initialize_wallet() -> Result<PrivateKey> {
    print!("Seed phrase: ");
    std::io::stdout().flush().unwrap();
    let password = read_password().unwrap();
    let secp = Secp256k1::new();
    let from_key = PrivateKey::from_words(&secp, &password, 0, 0).unwrap();
    Ok(from_key)
}

fn sender_public_key() -> PublicKey {
    let from_key = initialize_wallet().unwrap();
    get_public_key(&from_key)
}

pub fn get_public_key(from_key: &PrivateKey) -> PublicKey {
    let secp = Secp256k1::new();
    from_key.public_key(&secp)
}

pub fn validate_balance(network: &str, data: &utils::MultisendInstruction) -> Result<()> {
    let address = sender_public_key();
    let client = rpc_connection(network, "", 1.4).unwrap();
    for sender in &data.senders {
        let balance = get_token_balance(&client, &sender.coin, &address)?;
        println!(
            "Balance for {} is {}",
            sender.address,
            balance as f64 / TERRA_DECIMAL
        );
        if balance < (sender.amount * TERRA_DECIMAL) as u64 {
            return Err(Error::InvalidConfig(format!(
                "Address {} has a lower balance than send amount.",
                sender.address
            )));
        }
    }
    Ok(())
}

#[tokio::main]
async fn get_token_balance(
    client: &Terra,
    token_contract: &str,
    address: &PublicKey,
) -> Result<u64> {
    let balance_query = QueryMsg::Balance {
        address: address.account().expect("Balance Lookup Failed"),
    };

    let balance_uri = format!(
        "/terra/wasm/v1beta1/contracts/{}/store?query_msg={}",
        token_contract,
        balance_query.base64encode()
    );
    let balance_obj: QueryMsg = client
        .send_cmd(&balance_uri, None)
        .await
        .expect("Invalid Response");
    Ok(balance_obj.balance())
}

/// Balance Query for tokens
/// https://lcd.terra.dev/terra/wasm/v1beta1/contracts/{contract address}/store?query_msg={base 64 decode message}
/// https://docs.terraswap.io/docs/reference/token/
///
///
///
/// For transfer Implement https://github.com/PFC-Validator/terra-rust/blob/main/examples/do_swap.rs#L278
///
///
/// For MultiSend, will need to add MsgSendStruct
// https://bombay-lcd.terra.dev/terra/wasm/v1beta/contracts/terra1kc87mu460fwkqte29rquh4hc20m54fxwtsx7gp/store?query_msg=eyJiYWxhbmNlIjp7ImFkZHJlc3MiOiJ0ZXJyYTF2Z3B5YXoyMzJ3Y2hrdGdrYzg4Z3IyeGhxZGpxdjQ5aHE0dG52ayJ9fQ==
pub fn rpc_connection(network: &str, gas_price: &str, gas_adj: f64) -> Result<Terra> {
    let gas_opts =
        GasOptions::create_with_gas_estimate(gas_price, gas_adj).expect("Invalid Gas Parameters");
    let conn = match network {
        //Config options
        "devnet" => Terra::lcd_client(
            "https://bombay-lcd.terra.dev",
            "bombay-12",
            &gas_opts,
            Some(false),
        ),
        "mainnet" => Terra::lcd_client(
            "https://lcd.terra.dev",
            "columbus-5",
            &gas_opts,
            Some(false),
        ),
        _ => Terra::lcd_client(
            "https://bombay-lcd.terra.dev",
            "bombay-12",
            &gas_opts,
            Some(false),
        ),
    };
    Ok(conn)
}
