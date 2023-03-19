use std::io::Result;

fn main() -> Result<()> {
    let proto_root = "src/protos";
    let proto_files = ["image_processing.proto"];

    println!("cargo:rerun-if-changed=src/protos");
    prost_build::Config::new()
        .out_dir(proto_root)
        .compile_protos(&proto_files, &[proto_root])
}

