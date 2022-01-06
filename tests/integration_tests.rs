extern crate multisend;

use multisend::chains::Chain;
use multisend::chains::Solana;
mod common;

#[test]
fn test_validate_tx_amounts() {
    let data = common::setup(vec![0.10], vec![0.001, 0.02, 0.079]);
    let valid = multisend::utils::validate_tx_amounts(&data).unwrap();
    assert_eq!(valid, true)
}

#[test]
#[should_panic]
fn test_invalidate_tx_amounts() {
    let data = common::setup(vec![0.10], vec![0.001, 0.02, 0.07]);
    let _ = multisend::utils::validate_tx_amounts(&data).unwrap();
}

#[test]
#[should_panic]
fn test_invalidate_addresses() {
    // We're passing in no addresses
    let chain = Box::new(Solana {
        network: "mainnet".to_string(),
    });
    let data = common::setup(vec![0.10], vec![0.001, 0.02, 0.07]);
    let _ = chain.validate_addrs(&data).unwrap();
}
