use std::path::PathBuf;

fn main() {
    let root = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());

    println!(
        "cargo:rustc-link-arg-bins=-T{}",
        root.join("link.ld").to_str().unwrap()
    );
}
