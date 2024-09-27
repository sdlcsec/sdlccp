use std::{env, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    tonic_build::configure()
    .file_descriptor_set_path(out_dir.join("namespace_service.bin")) 
    .build_server(true)
    .compile(
        &["../proto/sdlc_control_plane/v1alpha1/namespace_service.proto"],
        &["../proto"],
    )?;
    Ok(())
}