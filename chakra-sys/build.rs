use std::{env, path::PathBuf, process::Command};

fn main() {
    let working_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
        .canonicalize()
        .unwrap();
    let liburing = working_dir.join("liburing");

    let build_dir = liburing.join("build");
    // Run the configure script to get the `config_host.h` file.
    Command::new("./configure")
        .args(&["--prefix", build_dir.to_str().unwrap()])
        .current_dir(liburing.clone())
        .output()
        .expect("configure script failed.");

    let source_dir = liburing.join("src");

    cc::Build::new()
        .file(source_dir.join("setup.c"))
        .file(source_dir.join("queue.c"))
        .file(source_dir.join("syscall.c"))
        .file(source_dir.join("register.c"))
        .include(build_dir.join("config_host.h"))
        .include(source_dir.join("include"))
        .flag("-g")
        .out_dir(env::var("CARGO_MANIFEST_DIR").unwrap())
        .compile("uring");
}
