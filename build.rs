fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rustc-link-arg=-Texamples/bare.ld");
}
