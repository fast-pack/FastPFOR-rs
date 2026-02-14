//! Build script for `FastPFOR-rs`.

/// Builds the C++ `FastPFOR` library and bridge when the `cpp` feature is enabled.
#[cfg(feature = "cpp")]
fn build_fastpfor() {
    use std::path::Path;

    assert!(
        Path::new("cpp/CMakeLists.txt").exists(),
        "FastPFOR submodule not initialized. Run `git submodule update --init`."
    );

    // Compile FastPFOR using CMake
    println!("cargo:rerun-if-changed=cpp");
    let lib_path = cmake::Config::new("cpp")
        .define("WITH_TEST", "OFF")
        .build()
        .join("lib");
    let lib_path = lib_path.to_str().unwrap();

    // Compile the bridge
    println!("cargo:rerun-if-changed=src/cpp/fastpfor_bridge.h");
    println!("cargo:rerun-if-changed=src/cpp/mod.rs");
    cxx_build::bridge("src/cpp/mod.rs")
        .include("cpp/headers")
        .include("src/cpp")
        .std("c++14")
        .compile("fastpfor_bridge");

    // Link the FastPFOR library - must be done after the bridge is compiled
    println!("cargo:rustc-link-search=native={lib_path}");
    println!("cargo:rustc-link-lib=static=FastPFOR");
}

/// Build script entry point.
fn main() {
    #[cfg(feature = "cpp")]
    build_fastpfor();
}
