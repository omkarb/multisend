extern crate multisend;

pub fn setup(senders: Vec<f64>, recipients: Vec<f64>) -> multisend::utils::MultisendInstruction {
    let mut s_instr = vec![];
    let mut r_instr = vec![];
    for i in senders {
        let sender = multisend::utils::TransferInstruction {
            address: "".to_string(),
            amount: i,
            coin: "sol".to_string(),
        };
        s_instr.push(sender)
    }
    for i in recipients {
        let sender = multisend::utils::TransferInstruction {
            address: "".to_string(),
            amount: i,
            coin: "sol".to_string(),
        };
        r_instr.push(sender)
    }

    multisend::utils::MultisendInstruction {
        senders: s_instr,
        recipients: r_instr,
    }
}
