fn main() {
    let dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    println!("cargo:rerun-if-changed=config/linker.ld");
    println!("cargo:rustc-link-arg-bins=-T{dir}/config/linker.ld");
}
