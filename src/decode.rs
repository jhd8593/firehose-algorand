use firehose_algorand::pb::Block;
use prost::Message;
use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("output.pb").expect("❌ Failed to open output.pb");
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).expect("❌ Failed to read output.pb");

    let block = Block::decode(&*buf).expect("❌ Failed to decode protobuf block");

    println!("✅ Decoded block round: {}", block.round);
    println!("📅 Timestamp: {}", block.timestamp);
    println!("🧾 Transactions:");
    for txn in block.transactions {
        println!(
            "- {} sent {} to {} (type: {})",
            txn.sender, txn.amount, txn.receiver, txn.txn_type
        );
    }
}
