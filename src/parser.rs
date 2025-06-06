use serde::Serialize;
use anyhow::Result;
use serde::Deserialize;
use prost::Message;


include!(concat!(env!("OUT_DIR"), "/firehose.algorand.rs")); // Generated Protobuf

use crate::pb;

// Conversion from internal Block to protobuf Block
impl From<Block> for pb::Block {
    fn from(b: Block) -> Self {
        pb::Block {
            round: b.round,
            timestamp: b.timestamp,
            transactions: b.transactions.into_iter().map(|t| pb::Transaction {
                txn_type: t.txn_type,
                sender: t.sender,
                receiver: t.receiver,
                amount: t.amount,
            }).collect(),
        }
    }
}


#[derive(Debug, Deserialize)]
pub struct AlgorandBlock {
    pub block: InnerBlock,
}

#[derive(Debug, Deserialize)]
pub struct InnerBlock {
    #[serde(rename = "rnd")]
    pub round: u64,
    #[serde(rename = "ts")]
    pub timestamp: u64,
    #[serde(default)]
    pub txns: Vec<TxnWrapper>,
}

#[derive(Debug, Serialize)]
pub struct BlockJson {
    pub round: u64,
    pub timestamp: u64,
    pub transactions: Vec<TransactionJson>,
}

#[derive(Debug, Serialize)]
pub struct TransactionJson {
    pub txn_type: String,
    pub sender: String,
    pub receiver: Option<String>,
    pub amount: u64,
}

#[derive(Debug, Deserialize)]
pub struct TxnWrapper {
    #[serde(rename = "txn")]
    pub txn: RawTransaction,
}

#[derive(Debug, Deserialize)]
pub struct RawTransaction {
    #[serde(rename = "type")]
    pub txn_type: String,
    pub snd: Option<String>,
    pub rcv: Option<String>,
    pub amt: Option<u64>,
}

pub async fn fetch_block_proto(round: u64) -> Result<Block, anyhow::Error> {
    let url = format!(
        "https://mainnet-api.algonode.cloud/v2/blocks/{}",
        round
    );
    let resp = reqwest::get(&url).await?.json::<AlgorandBlock>().await?;

    // Create protobuf block for binary output
    let proto_block = Block {
        round: resp.block.round,
        timestamp: resp.block.timestamp,
        transactions: resp.block.txns.iter().map(|t| {
            let txn = &t.txn;
            Transaction {
                txn_type: txn.txn_type.clone(),
                sender: txn.snd.clone().unwrap_or_default(),
                receiver: txn.rcv.clone().unwrap_or_default(),
                amount: txn.amt.unwrap_or(0),
            }
        }).collect(),
    };
    Ok(proto_block)
}

pub async fn fetch_block_and_output(round: u64) -> Result<()> {
    let url = format!(
        "https://mainnet-api.algonode.cloud/v2/blocks/{}",
        round
    );
    let resp = reqwest::get(&url).await?.json::<AlgorandBlock>().await?;

    // Create protobuf block for binary output
    let proto_block = Block {
        round: resp.block.round,
        timestamp: resp.block.timestamp,
        transactions: resp.block.txns.iter().map(|t| {
            let txn = &t.txn;
            Transaction {
                txn_type: txn.txn_type.clone(),
                sender: txn.snd.clone().unwrap_or_default(),
                receiver: txn.rcv.clone().unwrap_or_default(),
                amount: txn.amt.unwrap_or(0),
            }
        }).collect(),
    };

    // Output binary protobuf to stdout
    let mut buf = Vec::new();
    proto_block.encode(&mut buf)?;
    std::io::stdout().write_all(&buf)?;

    // Create JSON-serializable block for file output
    let json_block = BlockJson {
        round: resp.block.round,
        timestamp: resp.block.timestamp,
        transactions: resp.block.txns.into_iter().map(|t| {
            let txn = t.txn;
            TransactionJson {
                txn_type: txn.txn_type,
                sender: txn.snd.unwrap_or_default(),
                receiver: match txn.rcv {
    Some(ref r) if !r.is_empty() => Some(r.clone()),
    _ => None,
},
                amount: txn.amt.unwrap_or(0),
            }
        }).collect(),
    };

    // Output to JSON file
    use std::fs;
    use std::path::Path;
    use serde_json::to_string_pretty;
    let json_output = to_string_pretty(&json_block)?;
    let filename = format!("output/{}.json", round);
    fs::create_dir_all(Path::new("output"))?;
    fs::write(&filename, json_output)?;
    println!("📄 Output saved: output/{}.json", round);

    // Write per-transaction JSONL
    use std::fs::OpenOptions;
    use std::io::Write;
    use serde_json::json;
    let txns_filename = format!("output/{}.txns.jsonl", round);
    let mut txns_file = OpenOptions::new().create(true).write(true).truncate(true).open(&txns_filename)?;
    for txn in &json_block.transactions {
        serde_json::to_writer(&mut txns_file, &json!({ "round": round, "txn": txn }))?;
        writeln!(&mut txns_file)?;
    }
    println!("📄 Output saved: {}", txns_filename);

    Ok(())
}