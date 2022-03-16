pub mod solana;

use crate::{utils, Result};

pub struct Solana {
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
        // We send transactions in batches of 20 transfers because solana has a max data size
        // and after about twenty transfers we run out of space.
        for instruction_batch in instructions.chunks(20) {
            solana::send_transaction(&keypair, &self.network, instruction_batch.to_vec())
                .expect("Transaction Failed to Successfully Execute.");
        }
        Ok(())
    }

    fn validate_addrs(&self, data: &utils::MultisendInstruction) -> Result<()> {
        solana::validate_addrs(data)
    }

    fn validate_balance(&self, data: &utils::MultisendInstruction) -> Result<()> {
        solana::validate_balance(&self.network, data)
    }
}
