use clap::{crate_name, crate_version, App, AppSettings, Arg, ArgMatches, SubCommand};
use multisend::chains::{Chain, Solana, Terra};
use multisend::utils;
use multisend::Result;
use std::process;

fn run(matches: &ArgMatches) -> Result<()> {
    let chain: Box<dyn Chain>;
    let data = utils::read_instruction_json(matches.value_of("path").unwrap())?;
    let network = matches.value_of("network").unwrap_or("devnet").to_owned();
    // Implementing chains specific logic here.
    print!(
        "{:?}",
        matches
            .subcommand_matches("broadcast-transaction")
            .unwrap()
            .value_of("gas_price").
    );
    chain = match matches.value_of("chain").unwrap_or("terra") {
        "solana" => Box::new(Solana { network }),
        "terra" => {
            let gas_price = matches.value_of("gas_price").unwrap_or("").to_owned();
            let gas_adjustment = matches
                .value_of("gas_adjustment")
                .unwrap_or("1.4")
                .to_owned();
            let gas_adj: f64 = gas_adjustment
                .parse()
                .expect("Gas adjustment not a valid float value.");
            let memo = matches.value_of("memo").unwrap_or("").to_owned();
            print!("TEST: {}", memo);
            Box::new(Terra {
                network,
                gas_price,
                gas_adj,
                memo,
            })
        }
        _ => Box::new(Solana { network }),
    };
    let _results = match matches.subcommand() {
        ("broadcast-transaction", _) => execute_transaction(chain, &data),
        ("validate", _) => run_validate(chain, &data),
        _ => (),
    };
    Ok(())
}

fn run_validate(chain: Box<dyn Chain>, data: &utils::MultisendInstruction) {
    let _valid_amounts =
        utils::validate_tx_amounts(data).expect("Sender & Receiver amount mismatch");
    let _valid_addrs = chain
        .validate_addrs(data)
        .expect("Address Validation Error");
    let _valid_addrs = chain
        .validate_balance(data)
        .expect("Balance Validation Error");
    println!("Successfully validated.");
}

fn execute_transaction(chain: Box<dyn Chain>, data: &utils::MultisendInstruction) {
    chain
        .execute_transaction(data)
        .expect("Error sending transaction")
}

fn main() {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .arg(
            Arg::with_name("network")
                .short("n")
                .takes_value(true)
                .help("Network to send tx on: [mainnet-beta, testnet, devnet, localhost]"),
        )
        .arg(
            Arg::with_name("chain")
                .short("c")
                .takes_value(true)
                .help("Chain to send transaction on."),
        )
        .arg(
            Arg::with_name("path")
                .short("p")
                .takes_value(true)
                // .required(true)
                .help("Path for the transactions file."),
        )
        .subcommand(
            SubCommand::with_name("validate")
                .about("Verifies sender/receipiant amts. match and contains valid addresses."),
        )
        .subcommand(
            SubCommand::with_name("broadcast-transaction")
                .about("Send configured transaction")
                .arg(
                    Arg::with_name("gas_price")
                        .help("Gas price to pay for transactions [TERRA ONLY]")
                        .index(1),
                )
                .arg(
                    Arg::with_name("gas_adjustment")
                        .help("Gas price to use for transactions [TERRA ONLY]")
                        .default_value("1.4")
                        .index(2),
                )
                .arg(Arg::with_name("memo").help("Tranasction Memo").index(3)),
        )
        .get_matches();

    if let Err(e) = run(&matches) {
        println!("Application Error. {}", e);
        process::exit(1);
    }
}
