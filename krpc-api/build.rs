use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let out_dir = std::env::var("OUT_DIR").expect("OUT_DIR is not defined.");
    prost_build::compile_protos(&["../resources/api.proto"], &["../resources/"])?;
    std::fs::copy(format!("{}/{}", out_dir, "/krpc.api.rs"), "../resources/krpc.api.rs")?;
    Ok(())
}