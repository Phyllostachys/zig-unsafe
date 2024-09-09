fn main() {
    println!("cargo:rustc-link-lib=zig_unsafe_output");
    println!("cargo:rustc-link-search=native={}", std::env::var("CARGO_MANIFEST_DIR").unwrap());
}
