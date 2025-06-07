use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_dir = "proto";
    
    // Print the paths for debugging
    println!("cargo:warning=Proto directory: {:?}", std::fs::canonicalize(proto_dir)?);
    
    // Configure prost_build with version 0.12.6 settings
    let mut config = prost_build::Config::new();
    
    // Enable btree_map for all message fields that are maps
    config.btree_map(["."]);
    
    // Enable serde serialization/deserialization for all types
    config.type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]");
    
    // Add protoc argument for proto3 optional support
    config.protoc_arg("--experimental_allow_proto3_optional");
    
    // Tell cargo to invalidate the built crate whenever any of the proto files change
    println!("cargo:rerun-if-changed={}", proto_dir);
    
    // Define proto files to compile
    let proto_files = &[
        "proto/sf/algorand/type/v1/type.proto",
        "proto/algorand/firehose/v1/output.proto",
        "proto/block.proto",
    ];
    
    // Check if proto files exist and set up change tracking
    for proto_file in proto_files {
        let path = Path::new(proto_file);
        if !path.exists() {
            return Err(format!("Proto file not found: {}", proto_file).into());
        }
        println!("cargo:rerun-if-changed={}", proto_file);
    }
    
    // Compile all proto files
    config.compile_protos(proto_files, &[proto_dir])?;
    
    // Print success message
    println!("cargo:warning=Successfully generated protobuf bindings");
    
    Ok(())
}
