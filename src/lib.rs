pub mod pb {
    include!(concat!(env!("OUT_DIR"), "/firehose.algorand.rs"));
}
pub mod parser;
pub mod stream;
