# Raydium Liquidity CLI

A command-line tool to interact with Raydium AMM pools on the Solana blockchain. This application allows you to add or remove liquidity to a specified Raydium pool using your local Solana keypair.

---

## Features

- Add Liquidity: Deposit tokens into a Raydium pool to earn LP tokens.
- Remove Liquidity: Withdraw tokens from a Raydium pool by burning LP tokens.

---

## Prerequisites

- Rust & Cargo:  
  Ensure that you have Rust and Cargo installed.  
  Visit: https://www.rust-lang.org/tools/install

- Solana CLI & SPL-Token CLI (Optional):  
  The Solana CLI helps manage your configuration and keys, while the `spl-token` CLI assists in creating associated token accounts (ATAs).  
  Visit: https://docs.solana.com/cli/install-solana-cli-tools

- Funded Solana Keypair:  
  You must have a Solana keypair file (e.g. `~/.config/solana/id.json`) funded with sufficient SOL and any required tokens.  
  Recommended: Create a dedicated keypair for programmatic operations and fund it from your Phantom wallet.

---

## Installation

1. Clone the Repository:
   git clone https://github.com/guilhermeyoshida/raydium-liquidity-cli.git
   cd raydium-liquidity-cli

2. Build the Project:
   cargo build

   Once the build completes, the binary will be located in `target/debug`.

---

## Configuration

### Environment Variables

- `SOLANA_WALLET` (optional):  
  By default, the application uses `~/.config/solana/id.json` as the keypair. To specify a custom keypair path:
  export SOLANA_WALLET=~/.config/solana/custom-id.json

- Solana Cluster:  
  Ensure the Solana CLI is configured for the correct network:
  solana config set --url https://api.mainnet-beta.solana.com

  For devnet:
  solana config set --url devnet

---

## Usage

### View Help

Display all available commands and flags:
cargo run -- --help

### Example: Add Liquidity

1. Identify Pool Information:  
   Obtain the pool’s `ammId`, `coinMint`, `pcMint`, and `lpMint` from Raydium’s official pool configuration data.

2. Run the Command:  
   cargo run -- add <AMM_ID> <AMOUNT>

---

### Example: Remove Liquidity

1. Check LP Balance:  
   Ensure your LP ATA holds LP tokens from the pool.

2. Run the Command:  
   cargo run -- add <AMM_ID> <AMOUNT>

---

## Common Issues

- `InvalidProgramAddress`:  
  Ensure you are using the correct pool configuration data (`lpMint`, `coinMint`, `pcMint`) and that your ATAs are valid and funded.

- `Transaction simulation failed`:  
  Confirm that you have enough SOL and tokens in your ATAs, and verify that you’re on the correct network. Double-check the mint addresses.

---

## Security

- Keep your keypair and seed phrase private.
- Use dedicated wallets for programmatic operations to minimize risk.

---

## License

This project is licensed under the MIT License.

---

## Contact

For support, open an issue in the repository or contact the maintainers.
