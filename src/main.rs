use clap::{crate_name, crate_version, App, AppSettings, Arg, ArgMatches, SubCommand};
use multisend::chains::{Chain, Solana};
use multisend::utils;
use multisend::Result;
use std::process;

fn run(matches: &ArgMatches) -> Result<()> {
    let data = utils::read_instruction_json(matches.value_of("path").unwrap())?;
    let network = matches.value_of("network").unwrap_or("devnet").to_owned();
    let chain = match matches.value_of("network") {
        // Right now there's only one option here but we can add more later.
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
                .required(true)
                .help("Path for the transactions file."),
        )
        .subcommand(
            SubCommand::with_name("validate")
                .about("Verifies sender/receipiant amts. match and contains valid addresses."),
        )
        .subcommand(
            SubCommand::with_name("broadcast-transaction").about("Send configured transaction"),
        )
        .get_matches();

    if let Err(e) = run(&matches) {
        println!("Application Error. {}", e);
        process::exit(1);
    }
}
