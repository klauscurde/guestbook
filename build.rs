//use std::{env, path::PathBuf};

fn main() {
    tonic_build::compile_protos("proto/guestbook.proto")
        .unwrap_or_else(|e| panic!("Failed to compile protos {:?}", e));
}
