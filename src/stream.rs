use async_stream::stream;
use futures::Stream;
use crate::parser::fetch_block_proto;
use crate::pb::Block;
use tonic::Status;
use std::pin::Pin;

pub fn algorand_block_stream(start_round: u64, stop_round: u64) -> Pin<Box<dyn Stream<Item = Result<Block, Status>> + Send>> {
    Box::pin(stream! {
        for round in start_round..=stop_round {
            match fetch_block_proto(round).await {
                Ok(block) => yield Ok(block),
                Err(e) => yield Err(Status::internal(format!("parse error: {}", e))),
            }
        }
    })
}
