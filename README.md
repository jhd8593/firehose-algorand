<a href="https://www.streamingfast.io/">
	<img width="100%" src="https://github.com/streamingfast/substreams/blob/develop/docs/assets/substreams-banner.png" alt="StreamingFast Substreams Banner" />
</a>

# Algorand Firehose Substreams

A powerful Algorand blockchain indexing solution built with Substreams technology, developed for The Graph Network.

This project provides high-performance streaming and processing of Algorand blockchain data with built-in analytics capabilities.

## Features

- **Full Block Processing**: Extract complete Algorand block data including transactions, headers, and metadata
- **Transaction Analytics**: Focused extraction of sender addresses, amounts, and transaction types for analytics
- **File Output Support**: Save processed data to JSON files for further analysis
- **High Performance**: Leverages Substreams parallelization and streaming-first architecture
- **JWT Authentication**: Secure connection to The Graph's hosted services

## Quick Start

### Prerequisites

- Rust toolchain
- The Graph CLI (for authentication)
- Valid API key from The Graph

### Authentication Setup

1. Install The Graph CLI:
```bash
npm install -g @graphprotocol/graph-cli
```

2. Authenticate and get JWT token:
```bash
graph auth --product substreams https://playgrounds.substreams.network
```

3. Use the returned JWT token in your commands.

### Running Substreams

#### Basic Block Processing
```bash
# Stream to console
SUBSTREAMS_API_TOKEN=your_jwt_token ./substreams_new run algorand.spkg map_blocks -s 50500000 -t 50500010 -o json -e mainnet.eth.streamingfast.io:443

# Save to file (using shell redirection)
SUBSTREAMS_API_TOKEN=your_jwt_token ./substreams_new run algorand.spkg map_blocks -s 50500000 -t 50500010 -o json -e mainnet.eth.streamingfast.io:443 > ./output/blocks.json
```

#### Transaction Analytics (Recommended for Analytics)
```bash
# Stream transaction analytics to console
SUBSTREAMS_API_TOKEN=your_jwt_token ./substreams_new run algorand.spkg map_transaction_analytics -s 50500000 -t 50500010 -o json -e mainnet.eth.streamingfast.io:443

# Save analytics to file (using shell redirection)
SUBSTREAMS_API_TOKEN=your_jwt_token ./substreams_new run algorand.spkg map_transaction_analytics -s 50500000 -t 50500010 -o json -e mainnet.eth.streamingfast.io:443 > ./output/analytics.json
```

## Available Modules

### 1. `map_blocks`
Extracts complete block data including:
- Block headers (round, timestamp, previous block hash)
- All transactions with full details
- Transaction IDs and metadata

**Output Format**: Structured protobuf data with complete block information

### 2. `map_transaction_analytics` ⭐ **Recommended for Analytics**
Extracts focused transaction data for analytics:
- Sender addresses
- Transaction amounts
- Transaction types (payment, asset_transfer, etc.)
- Block round and timestamp

**Sample Output**:
```json
[
  {
    "sender": "ABC123...",
    "amount": 1000000,
    "type": "payment",
    "round": 50500001,
    "timestamp": 1640995200
  },
  {
    "sender": "XYZ789...",
    "amount": 50000,
    "type": "asset_transfer",
    "round": 50500001,
    "timestamp": 1640995200
  }
]
```

## File Output Options

### Console Output
```bash
-o json
```

### File Output (using shell redirection)
```bash
-o json > ./output/filename.json
```

The `output/` directory is automatically created for storing results.

## Parameters

- `-s, --start-block`: Starting block number
- `-t, --stop-block`: Ending block number (exclusive)
- `-o, --output`: Output format (`json`) or file path
- `-e, --endpoint`: Substreams endpoint URL

## Example Use Cases

### 1. Transaction Volume Analysis
```bash
# Extract 1000 blocks of transaction data
SUBSTREAMS_API_TOKEN=your_jwt_token ./substreams_new run algorand.spkg map_transaction_analytics -s 50500000 -t 50501000 -o json -e mainnet.eth.streamingfast.io:443 > ./output/volume_analysis.json
```

### 2. Large Address Activity Tracking
```bash
# Process recent blocks for large transactions
SUBSTREAMS_API_TOKEN=your_jwt_token ./substreams_new run algorand.spkg map_transaction_analytics -s 50600000 -t 50600100 -o json -e mainnet.eth.streamingfast.io:443 > ./output/large_transactions.json
```

### 3. Historical Data Export
```bash
# Export complete block data for archival
SUBSTREAMS_API_TOKEN=your_jwt_token ./substreams_new run algorand.spkg map_blocks -s 50000000 -t 50010000 -o json -e mainnet.eth.streamingfast.io:443 > ./output/historical_blocks.json
```

## Building from Source

```bash
# Build the Rust module
cargo build --target wasm32-unknown-unknown --release

# Package the substream
substreams pack substreams.yaml
```

## 📷 Demo

A live demonstration recording is available in the repository:

**File**: `algorand-substreams-demo.cast`

To view the demo online, upload it to asciinema.org:
```bash
asciinema upload algorand-substreams-demo.cast
```

The demo shows:
- ✅ JWT token authentication working
- ✅ Substreams connecting to The Graph's endpoint  
- ✅ Real-time processing of Algorand blocks (50500000-50500003)
- ✅ JSON output streaming to console
- ✅ TraceID and processing status

*Live demonstration of streaming Algorand blockchain data with JWT authentication and real-time processing*

## Authentication Status

✅ **JWT Authentication**: Fully supported and working
✅ **Compression Support**: Modern gzip/zstd compression enabled
✅ **File Output**: JSON file export supported
✅ **Analytics Module**: Transaction analytics extraction ready

## Documentation

- [Substreams Documentation](https://substreams.streamingfast.io)
- [The Graph Documentation](https://thegraph.com/docs/)
- [Algorand Developer Resources](https://developer.algorand.org/)

## Contributing

Please refer to the [StreamingFast contribution guide](https://github.com/streamingfast/streamingfast/blob/master/CONTRIBUTING.md) for contribution guidelines.

## License

[Apache 2.0](LICENSE)
