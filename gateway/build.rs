fn main() {
    println!("cargo:rustc-link-arg-bins=-zcommon-page-size=16384");
    println!("cargo:rustc-link-arg-bins=-zmax-page-size=16384");
}
