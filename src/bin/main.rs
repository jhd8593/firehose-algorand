use firehose_algorand::stream::algorand_block_stream;
use clap::{Parser, ValueEnum};
use futures::StreamExt; // for .next().await
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use serde_json;

/// Algorand block streamer that saves blocks to files
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Starting block number (inclusive)
    #[arg(long)]
    start_block: u64,

    /// Ending block number (inclusive)
    #[arg(long)]
    stop_block: u64,

    /// Output directory for block files
    #[arg(long, default_value = "./output")]
    output_dir: PathBuf,

    /// Format output JSON with pretty printing
    #[arg(long)]
    json_pretty: bool,

    /// Set the logging level
    #[arg(long, default_value = "info")]
    log_level: LogLevel,
}

#[derive(ValueEnum, Clone, Debug)]
enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

impl std::fmt::Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    
    // Initialize logger with the specified log level
    env_logger::Builder::new()
        .filter_level(match args.log_level {
            LogLevel::Error => log::LevelFilter::Error,
            LogLevel::Warn => log::LevelFilter::Warn,
            LogLevel::Info => log::LevelFilter::Info,
            LogLevel::Debug => log::LevelFilter::Debug,
            LogLevel::Trace => log::LevelFilter::Trace,
        })
        .init();
        
    let start = args.start_block;
    let stop = args.stop_block;

    // Initialize benchmarking
    let start_time = std::time::Instant::now();
    let mut block_count = 0;
    let mut total_txns = 0;

    println!("Streaming blocks {} to {}", start, stop);
    let mut block_stream = algorand_block_stream(start, stop);
    while let Some(result) = block_stream.next().await {
        match result {
            Ok(block) => {
                println!("✅ Block {} ({} txns)", block.round, block.transactions.len());
                // Ensure output directory exists
                if !args.output_dir.exists() {
                    if let Err(e) = fs::create_dir_all(&args.output_dir) {
                        log::error!("Failed to create output directory: {}", e);
                        continue;
                    }
                }
                
                // Write block as JSON
                let filename = args.output_dir.join(format!("{}.json", block.round));
                match File::create(&filename) {
                    Ok(mut file) => {
                        let result = if args.json_pretty {
                            serde_json::to_string_pretty(&block)
                        } else {
                            serde_json::to_string(&block)
                        };
                        
                        match result {
                            Ok(json) => {
                                if let Err(e) = file.write_all(json.as_bytes()) {
                                    log::error!("Failed to write block {}: {}", block.round, e);
                                } else {
                                    // Update counters
                                    block_count += 1;
                                    total_txns += block.transactions.len();
                                    
                                    log::info!("✅ Block {} ({} txns) saved to {}", 
                                        block.round, 
                                        block.transactions.len(),
                                        filename.display()
                                    );
                                    
                                    // Log progress every 100 blocks
                                    if block_count % 100 == 0 {
                                        let elapsed = start_time.elapsed().as_secs_f64();
                                        let blocks_per_sec = block_count as f64 / elapsed;
                                        log::info!("🔄 Processed {} blocks ({} txns) - {:.2} blocks/s", 
                                            block_count, 
                                            total_txns,
                                            blocks_per_sec
                                        );
                                    }
                                }
                            },
                            Err(e) => {
                                log::error!("Failed to serialize block {}: {}", block.round, e);
                            }
                        }
                    },
                    Err(e) => {
                        log::error!("Failed to create file for block {}: {}", block.round, e);
                    }
                }
            },
            Err(e) => {
                eprintln!("❌ Error streaming block: {}", e);
            }
        }
    }
    
    // Log final summary
    let elapsed = start_time.elapsed().as_secs_f64();
    let avg_txns_per_block = if block_count > 0 {
        total_txns as f64 / block_count as f64
    } else {
        0.0
    };
    let blocks_per_sec = block_count as f64 / elapsed;
    let txns_per_sec = total_txns as f64 / elapsed;

    log::info!(
        "\n✅ Completed: {} blocks ({} txns) in {:.1}s (Avg: {:.1} txns/block, {:.1} blocks/s, {:.1} txns/s)",
        block_count,
        total_txns,
        elapsed,
        avg_txns_per_block,
        blocks_per_sec,
        txns_per_sec
    );
}
