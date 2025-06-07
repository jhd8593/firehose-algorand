//! # Firehose Algorand Substream
//! 
//! This module implements a Substream for processing Algorand blockchain data.

// Include the generated protobuf code
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

// Include generated protobuf modules
pub mod sf_algorand_type_v1 {
    #![allow(clippy::all)]
    #![allow(warnings)]
    #![allow(rustdoc::invalid_html_tags)]
    #![allow(rustdoc::broken_intra_doc_links)]
    include!(concat!(env!("OUT_DIR"), "/sf.algorand.r#type.v1.rs"));
}

pub mod algorand_firehose_v1 {
    #![allow(clippy::all)]
    #![allow(warnings)]
    #![allow(rustdoc::invalid_html_tags)]
    #![allow(rustdoc::broken_intra_doc_links)]
    include!(concat!(env!("OUT_DIR"), "/algorand.firehose.v1.rs"));
}

pub mod firehose_algorand {
    #![allow(clippy::all)]
    #![allow(warnings)]
    #![allow(rustdoc::invalid_html_tags)]
    #![allow(rustdoc::broken_intra_doc_links)]
    include!(concat!(env!("OUT_DIR"), "/firehose.algorand.rs"));
}

pub mod substreams;
pub mod stream;
pub mod parser;
pub mod decode;

// Create pb module alias for protobuf types
pub mod pb {
    // Use the firehose_algorand types as the primary ones (from block.proto)
    pub use crate::firehose_algorand::{Block, Transaction};
}

// Re-export commonly used types (without glob imports to avoid conflicts)
pub use sf_algorand_type_v1 as sf_algorand;
pub use algorand_firehose_v1 as firehose;
pub use firehose_algorand as block;

// Required for Substreams WASM runtime
#[no_mangle]
pub extern "C" fn __malloc(size: usize) -> *mut u8 {
    unsafe {
        std::alloc::alloc(std::alloc::Layout::from_size_align(size, 1).unwrap())
    }
}

#[no_mangle]
pub extern "C" fn __free(ptr: *mut u8) {
    unsafe {
        std::alloc::dealloc(ptr, std::alloc::Layout::from_size_align(1, 1).unwrap());
    }
}

#[no_mangle]
pub extern "C" fn __realloc(ptr: *mut u8, size: usize) -> *mut u8 {
    unsafe {
        std::alloc::realloc(ptr, std::alloc::Layout::from_size_align(1, 1).unwrap(), size)
    }
}
