fn main() {
    let _build = cxx_build::bridge("lib.rs");
    println!("cargo:rerun-if-changed=lib.rs");
}
