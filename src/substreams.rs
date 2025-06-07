use anyhow::{Error, anyhow};
use substreams::pb::substreams::Clock;
use crate::sf_algorand_type_v1;
use crate::algorand_firehose_v1;
use crate::firehose_algorand;
use serde_json::json;

#[substreams::handlers::map]
pub fn map_blocks(blk: sf_algorand_type_v1::Block) -> Result<algorand_firehose_v1::Output, Error> {
    // Extract block header information
    let header = blk.header.ok_or_else(|| anyhow!("Block header is missing"))?;
    
    // Convert transactions to the expected format
    let transactions = blk.transactions
        .into_iter()
        .map(|tx| {
            let header = tx.header.as_ref().cloned().unwrap_or_default();
            
            // Convert transaction type enum to string
            let tx_type = match tx.r#type() {
                sf_algorand_type_v1::transaction::Type::Payment => "payment",
                sf_algorand_type_v1::transaction::Type::KeyRegistration => "keyreg",
                sf_algorand_type_v1::transaction::Type::AssetConfig => "acfg",
                sf_algorand_type_v1::transaction::Type::AssetTransfer => "axfer",
                sf_algorand_type_v1::transaction::Type::AssetFreeze => "afrz",
                sf_algorand_type_v1::transaction::Type::ApplicationCall => "appl",
            };
            
            // Create a new transaction in the firehose format
            algorand_firehose_v1::Transaction {
                id: hex::encode(&header.id),
                r#type: tx_type.to_string(),
                sender: hex::encode(&header.sender),
                fee: header.fee,
                first_valid: header.first_valid,
                last_valid: header.last_valid,
                note: String::from_utf8_lossy(&tx.note).to_string(),
                group: hex::encode(&tx.group),
            }
        })
        .collect::<Vec<_>>();

    // Extract transaction IDs
    let transaction_ids = transactions
        .iter()
        .map(|tx| tx.id.clone())
        .collect();

    // Create the output message
    let output = algorand_firehose_v1::Output {
        round: header.round,
        timestamp: header.timestamp as u64,
        transaction_ids,
        previous_block_hash: hex::encode(&header.previous_block_hash),
        transactions,
    };

    Ok(output)
}

// Analytics-focused map function that extracts transaction sender and amount data
#[substreams::handlers::map]
pub fn map_transaction_analytics(blk: sf_algorand_type_v1::Block) -> Result<String, Error> {
    let header = blk.header.ok_or_else(|| anyhow!("Block header is missing"))?;
    
    let mut analytics_data = Vec::new();
    
    for tx in blk.transactions {
        let tx_header = tx.header.as_ref().cloned().unwrap_or_default();
        
        // Extract payment amount based on transaction type
        let amount = match tx.r#type() {
            sf_algorand_type_v1::transaction::Type::Payment => {
                // For payment transactions, extract the amount from payment field
                tx.payment.map(|p| p.amount).unwrap_or(0)
            },
            sf_algorand_type_v1::transaction::Type::AssetTransfer => {
                // For asset transfers, extract the amount from asset transfer field
                tx.asset_transfer.map(|at| at.amount).unwrap_or(0)
            },
            _ => {
                // For other transaction types, use fee as the "amount"
                tx_header.fee
            }
        };
        
        // Only include transactions with meaningful amounts
        if amount > 0 {
            let analytics_entry = json!({
                "sender": hex::encode(&tx_header.sender),
                "amount": amount,
                "type": match tx.r#type() {
                    sf_algorand_type_v1::transaction::Type::Payment => "payment",
                    sf_algorand_type_v1::transaction::Type::AssetTransfer => "asset_transfer",
                    sf_algorand_type_v1::transaction::Type::KeyRegistration => "key_registration",
                    sf_algorand_type_v1::transaction::Type::AssetConfig => "asset_config",
                    sf_algorand_type_v1::transaction::Type::AssetFreeze => "asset_freeze",
                    sf_algorand_type_v1::transaction::Type::ApplicationCall => "app_call",
                },
                "round": header.round,
                "timestamp": header.timestamp
            });
            
            analytics_data.push(analytics_entry);
        }
    }
    
    // Return JSON string of the analytics data
    Ok(serde_json::to_string_pretty(&analytics_data)?)
}

// Add any helper functions here if needed
