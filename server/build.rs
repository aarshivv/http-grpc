use std::{env, path::PathBuf};

fn main() {
    // Path to your .proto file
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    tonic_build::configure()
        .file_descriptor_set_path(out_dir.join("taskservice_descriptor.bin"))
        .compile_protos(
            &["proto/task_service.proto"], // Paths to your proto files
            &["proto"],                      // Include directories, if any (e.g., for imports)
        )
        .expect("Failed to compile proto files");
}
