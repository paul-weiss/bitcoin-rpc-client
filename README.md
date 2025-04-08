# Bitcoin RPC Client

A Rust-based Bitcoin RPC client that demonstrates various Bitcoin Core RPC calls in regtest mode.

## Features

- Connects to Bitcoin Core in regtest mode
- Demonstrates various RPC calls including:
  - Blockchain information
  - Wallet information
  - Network information
  - Mining information
  - Mempool information
  - Block information
  - Peer information
  - Fee estimates

## Prerequisites

- Rust (latest stable version)
- Bitcoin Core (latest version)
- Cargo (Rust's package manager)

## Installation

1. Clone the repository:
```bash
git clone <your-repo-url>
cd bitcoin-rpc-client
```

2. Build the project:
```bash
cargo build
```

## Configuration

1. Update the credentials in `bitcoin.conf`:
```conf
rpcuser=your_username
rpcpassword=your_password
```

2. Update the same credentials in `src/main.rs`:
```rust
Auth::UserPass("your_username".to_string(), "your_password".to_string())
```

## Usage

1. Start Bitcoin Core in regtest mode:
```bash
bitcoind -conf=./bitcoin.conf
```

2. Generate initial blocks (required for regtest):
```bash
bitcoin-cli -conf=./bitcoin.conf generate 101
```

3. Run the RPC client:
```bash
cargo run
```

## Project Structure

```
bitcoin-rpc-client/
├── src/
│   └── main.rs          # Main application code
├── regtest-data/        # Bitcoin Core regtest data directory
├── bitcoin.conf         # Bitcoin Core configuration
├── Cargo.toml           # Rust project configuration
└── README.md            # This file
```

## Available RPC Calls

The client demonstrates the following RPC calls:

- `get_blockchain_info()` - Get blockchain information
- `get_wallet_info()` - Get wallet information
- `get_network_info()` - Get network information
- `get_mining_info()` - Get mining information
- `get_mempool_info()` - Get mempool information
- `get_best_block_hash()` - Get the best block hash
- `get_block_count()` - Get the current block count
- `get_difficulty()` - Get the current difficulty
- `get_connection_count()` - Get the number of connections
- `get_peer_info()` - Get peer information
- `get_raw_mempool()` - Get raw mempool information
- `get_mempool_entry()` - Get specific mempool entry
- `get_block_template()` - Get block template for mining
- `estimate_smart_fee()` - Get estimated fee

## Regtest Mode

This project uses Bitcoin's regtest mode, which has the following characteristics:

- Port 18443 for RPC (instead of 8332)
- No need to download the blockchain
- Blocks can be generated instantly
- Very low difficulty
- Ability to create as many bitcoins as needed
- Perfect for testing and development

## Troubleshooting

1. If you get connection errors:
   - Ensure Bitcoin Core is running
   - Check if the RPC credentials match in both files
   - Verify the regtest port (18443) is correct

2. If you get "Wallet file not specified" errors:
   - Make sure the wallet is enabled in bitcoin.conf
   - Try creating a new wallet: `bitcoin-cli -conf=./bitcoin.conf createwallet "testwallet"`

## Contributing

Feel free to submit issues and enhancement requests!

## License

This project is licensed under the MIT License - see the LICENSE file for details. 