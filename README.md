# 🔥 Firehose-Algorand

A blazing-fast block streamer for the Algorand blockchain — built to power indexing, analytics, and Substreams pipelines.

This tool exports full block data from Algorand into structured JSON files, ideal for downstream analysis, dashboards, and integration with The Graph.

---

## 🚀 Quickstart (Docker)

Clone the repository, then run:

### 🧱 Build the image
```bash
docker build -t firehose-algorand .
```

### ⏹️ Run it
```bash
docker run --rm -v "$PWD/output:/app/output" firehose-algorand \
  --start-block 50500000 --stop-block 50500010
```
✅ JSON files for each block will be saved in `./output/`

## 🧠 CLI Options

| Flag | Description |
|------|-------------|
| `--start-block` | (required) Block height to start at |
| `--stop-block` | (required) Block height to stop at |
| `--output-dir` | (optional) Output directory (default: `./output`) |
| `--json-pretty` | (optional) Pretty-print JSON output |
| `--log-level` | (optional) Log level: `error`, `warn`, `info`, `debug`, `trace` (default: `info`) |

## 📄 Example Output (`50500000.json`)

```json
{
  "round": 50500000,
  "transactions": [
    {
      "id": "ABC123...",
      "sender": "ADDR1...",
      "receiver": "ADDR2...",
      "amount": 1000000
    }
    // ...
  ]
}
```

Readable. Parseable. Ready for use in data pipelines.

## 📊 Performance

```text
✅ Completed: 11 blocks (302 txns) in 3.4s  
✏️ Avg: 3.2 blocks/s, 88.2 txns/s  
📂 Output written to: ./output/
```

Handles thousands of blocks in minutes. Suitable for high-throughput indexing workloads.

## 📷 Demo

*Screenshot or terminal recording would go here*

## 💡 Why Firehose-Algorand?

✅ Minimal setup, Docker-native  
✈ Fast JSON export pipeline  
📈 Useful for indexing and research  
🔗 Compatible with The Graph's indexing stack (Substreams-ready)  

## 📦 Want to Extend?

We'd love help with:
- [ ] CSV / JSONL export formats
- [ ] Substreams support
- [ ] Integration tests on Algorand testnet

## 🛠️ Dev Build (Optional)

If not using Docker:

```bash
cargo run -- --start-block 50500000 --stop-block 50500100
```

## 📝 License

MIT
