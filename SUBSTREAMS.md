# Algorand Firehose Substreams

This package provides Substreams integration for Algorand blockchain data, allowing you to index and process Algorand blocks and transactions efficiently.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version)
- [Substreams CLI](https://substreams.streamingfast.io/getting-started/installing-the-cli)
- [Protocol Buffers](https://grpc.io/docs/protoc-installation/) compiler

## Building the Substreams Package

1. Install the required Rust target for WebAssembly:
   ```bash
   rustup target add wasm32-unknown-unknown
   ```

2. Build the WebAssembly module:
   ```bash
   cargo build --release --target wasm32-unknown-unknown
   ```

3. Generate the Substreams package:
   ```bash
   substreams protogen
   substreams build
   ```

   This will generate a `.spkg` file in your project directory.

## Running the Substreams

### Local Development

1. Start a local Firehose-enabled Algorand node or connect to a remote one.

2. Run the Substreams:
   ```bash
   substreams run -e <endpoint> substreams.yaml map_blocks
   ```

   Replace `<endpoint>` with your Firehose Algorand endpoint (e.g., `mainnet.algodev.network:443`).

### Available Modules

- `map_blocks`: Processes Algorand blocks and extracts transaction data
- `db_out`: Outputs database changes for storing in a database

## Package Structure

- `substreams.yaml`: Substreams manifest file
- `proto/`: Protocol Buffer definitions
  - `algorand/firehose/v1/output.proto`: Custom output types
- `src/substreams.rs`: Substreams module implementation

## Output Format

The Substreams outputs data in the following format:

```protobuf
message Output {
  uint64 round = 1;
  repeated string transaction_ids = 2;
  uint64 timestamp = 3;
  string previous_block_hash = 4;
  repeated Transaction transactions = 5;
}

message Transaction {
  string id = 1;
  string type = 2;
  string sender = 3;
  uint64 fee = 4;
  uint64 first_valid = 5;
  uint64 last_valid = 6;
  string note = 7;
  string group = 8;
}
```

## Database Schema

The `db_out` module outputs database changes for the following tables:

### blocks
- `id`: Block round number (primary key)
- `timestamp`: Block timestamp
- `previous_block_hash`: Hash of the previous block
- `transaction_count`: Number of transactions in the block

### transactions
- `id`: Transaction ID (primary key)
- `block_round`: Reference to the block containing this transaction
- `type`: Transaction type
- `sender`: Sender's address
- `fee`: Transaction fee
- `first_valid`: First valid round
- `last_valid`: Last valid round
- `note`: Transaction note
- `group`: Transaction group ID

## License

[Your License Here]
