pub mod solana;
pub mod terra;

use crate::{utils, Result};

pub struct Solana {
    pub network: String,
}

pub struct Terra {
    pub network: String,
}

pub trait Chain {
    fn new(&self, name: String) -> Self
    where
        Self: Sized;
    fn execute_transaction(&self, data: &utils::MultisendInstruction) -> Result<()>;
    fn validate_addrs(&self, data: &utils::MultisendInstruction) -> Result<()>;
    fn validate_balance(&self, data: &utils::MultisendInstruction) -> Result<()>;
}

impl Chain for Solana {
    fn new(&self, network: String) -> Solana {
        Solana { network: network }
    }

    fn execute_transaction(&self, data: &utils::MultisendInstruction) -> Result<()> {
        // initialize wallet with seed phrase + optional derivation path.
        let keypair = solana::initialize_wallet("wallet", None).unwrap();
        // build instructions
        let instructions = solana::build_transfer_instruction(&keypair, data);
        // send transaction
        solana::send_transaction(&keypair, &self.network, instructions)
    }

    fn validate_addrs(&self, data: &utils::MultisendInstruction) -> Result<()> {
        solana::validate_addrs(data)
    }

    fn validate_balance(&self, data: &utils::MultisendInstruction) -> Result<()> {
        solana::validate_balance(&self.network, data)
    }
}

impl Chain for Terra {
    fn new(&self, network: String) -> Terra {
        Terra { network: network }
    }

    fn execute_transaction(&self, data: &utils::MultisendInstruction) -> Result<()> {
        // initialize wallet with seed phrase + optional derivation path.
        let from_key = terra::initialize_wallet().unwrap();
        let public_key = terra::get_public_key(&from_key);

        let msgs = terra::build_transfer_msgs(&public_key, data);
        terra::send_transaction(&self.network, from_key, msgs)
            .expect("Sending Transaction failed.");
        Ok(())
    }

    fn validate_addrs(&self, data: &utils::MultisendInstruction) -> Result<()> {
        Ok(())
    }

    fn validate_balance(&self, data: &utils::MultisendInstruction) -> Result<()> {
        terra::validate_balance(&self.network, data)
    }
}
