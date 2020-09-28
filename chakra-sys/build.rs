use std::{
    env,
    io::{stdout, Write},
    path::PathBuf,
    process::Command,
};

fn main() {
    println!("cargo:rustc-link-lib=uring");
    println!("cargo:rustc-link-search=build");

    let working_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
        .canonicalize()
        .unwrap();
    let liburing = working_dir.join("liburing");

    let build_dir = liburing.join("build");
    let build_arg = format!("--prefix={}", build_dir.to_str().unwrap());

    // Run the configure script to get the `config_host.h` file.
    let output = Command::new("./configure")
        .args(&[&build_arg])
        .current_dir(liburing.clone())
        .output()
        .expect("configure script failed.");

    stdout()
        .lock()
        .write_all(&output.stdout)
        .expect("Unable to write process output to stdout");
    println!();

    let source_dir = liburing.join("src");
    let output_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    cc::Build::new()
        .file(source_dir.join("setup.c"))
        .file(source_dir.join("queue.c"))
        .file(source_dir.join("syscall.c"))
        .file(source_dir.join("register.c"))
        .include(build_dir.join("config_host.h"))
        .include(source_dir.join("include"))
        .flag("-g")
        .out_dir(output_dir.join("build"))
        .compile("uring");
}
