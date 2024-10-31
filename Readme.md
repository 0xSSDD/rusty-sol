# Solana Transaction Fetcher

A basic Rust application that demonstrates how to fetch transaction details from the Solana blockchain.

## What it does

This simple application:
1. Connects to Solana's devnet
2. Takes a transaction signature
3. Retrieves and displays the full transaction details

## Code Overview

### Key Components

- **RPC Client**: Establishes connection to Solana's devnet
- **Transaction Signature**: Uses a hardcoded signature (can be modified)
- **Error Handling**: Uses `anyhow` for robust error management
- **Data Parsing**: Converts transaction data into readable format

### Dependencies

```toml
solana-client = "1.17"           # For RPC communication
solana-sdk = "1.17"              # Core Solana functionality
solana-transaction-status = "1.17.10"  # Transaction data structures
anyhow = "1.0"                   # Error handling
```

## Usage

1. Build the project:
```bash
cargo build
```

2. Run the application:
```bash
cargo run
```

The output will display detailed transaction information including:
- Block time
- Transaction fee
- Account interactions
- Program logs
- Status confirmation

## Example Output

The transaction details are printed in a debug format (`{:#?}`), showing the complete transaction structure including:
- Meta information
- Transaction instructions
- Signatures
- Account updates
