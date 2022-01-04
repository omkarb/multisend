# multisend

### Summary
Multisend is a tool built to allow users to conveniently send assets to a large number of assets quickly and efficiently. 

The Flipside Finance/Operations team sends out hundreds of payments on a number of different chains (Ethereum, Terra, Solana, Algorand) each week to users that complete the our bounties. 

This tool currently supports payments on Solana; we'll add Terra capabilities in the near-term.


### On Deck: 
1. Initial test cases.
2. Add Terra native currency functionality.
3. Optionally accepting a keyfile instead of a std.in seed phrase.
4. Add Terra CW-20 tokens functionality.

### USAGE:
    multisend [OPTIONS] -p <path> <SUBCOMMAND>

#### FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

#### OPTIONS:
    -c <chain>          Chain to send transaction on: [solana]
    -n <network>        Network to send tx on: [mainnet-beta, testnet, devnet, localhost]
    -p <path>           Path for the transactions file.

#### SUBCOMMANDS:
    broadcast-transaction    Send configured transaction
    help                     Prints this message or the help of the given subcommand(s)
    validate                 Verifies sender/receipiant amts. match and contains valid addresses.


### Workflow
1. Expected JSON format
```
{
    "recipients": [
	    {
            "address": "{}",
            "amount": 0.09,
            "coin": "SOL"
        },
	    {
            "address": "{}",
            "amount": 0.04,
            "coin": "SOL"
        }

    ],
    "senders": [
        {
            "address": "{}",
            "amount": 0.13,
            "coin": "SOL"
        }
    ]
}
```
2. `./multisend -p {path} validate` to ensure json is valid, amounts are correct, and all addresses are valid.
3. `./multisend -p {} -n {mainnet} broadcast-transaction` to execute the transaction.