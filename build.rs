use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut config = prost_build::Config::new();
    
    // Configure serde for all messages
    config
        .out_dir(std::env::var("OUT_DIR").unwrap())
        .compile_well_known_types()
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .type_attribute(".", "#[serde(rename_all = \"camelCase\")]");
    
    // Configure specific field attributes
    config.type_attribute("firehose.algorand.Transaction.txn_type", "#[serde(rename = \"txnType\")]");
    
    // Get the path to the proto directory
    let proto_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap()).join("proto");
    let proto_file = proto_dir.join("block.proto");
    
    // Tell Cargo to re-run if the proto file changes
    println!("cargo:rerun-if-changed={}", proto_file.display());
    
    // Compile the proto file
    config.compile_protos(&[proto_file], &[proto_dir])?;
    
    Ok(())
}
