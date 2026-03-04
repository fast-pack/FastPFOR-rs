//! Build script for `FastPFOR-rs`.

/// Builds the C++ `FastPFOR` library and bridge when the `cpp` feature is enabled.
#[cfg(feature = "cpp")]
fn build_fastpfor() {
    use std::env;
    use std::path::Path;

    assert!(
        Path::new("cpp/CMakeLists.txt").exists(),
        "FastPFOR submodule not initialized. Run `git submodule update --init`."
    );

    // Compile FastPFOR using CMake
    println!("cargo:rerun-if-changed=cpp");
    let cmake_out = cmake::Config::new("cpp").define("WITH_TEST", "OFF").build();
    let lib_path = cmake_out.join("lib");
    let lib_path = lib_path.to_str().unwrap();

    // Compile the bridge
    println!("cargo:rerun-if-changed=src/cpp/fastpfor_bridge.h");
    println!("cargo:rerun-if-changed=src/cpp/mod.rs");
    let mut bridge = cxx_build::bridge("src/cpp/mod.rs");
    bridge
        .include("cpp/headers")
        .include("src/cpp")
        .std("c++14");

    // On ARM/aarch64, FastPFOR headers include SIMDe shims for SSE intrinsics.
    // CMake fetches SIMDe to build FastPFOR itself, but the Rust/CXX bridge is a
    // separate compilation unit and needs the same compile definition, plus an
    // include path if CMake fetched SIMDe into the build tree.
    if env::var("CARGO_CFG_TARGET_ARCH").is_ok_and(|arch| arch == "aarch64") {
        // Mirror `cpp/cmake_modules/simde.cmake` for the bridge TU:
        // FastPFOR headers use SSE names directly (e.g. __m128i, _mm_*),
        // so we need SIMDe's native aliases enabled here as well, regardless of
        // where the SIMDe headers are provided from.
        bridge.define("SIMDE_ENABLE_NATIVE_ALIASES", None);

        let simde_include = cmake_out.join("build").join("_deps").join("simde-src");
        if simde_include.exists() {
            bridge.include(simde_include);
        } else {
            println!(
                "cargo:warning=SIMDe headers were not found in CMake build output; \
                 ensure SIMDe is available on the include path if bridge compilation fails."
            );
        }
    }

    bridge.compile("fastpfor_bridge");

    // Link the FastPFOR library - must be done after the bridge is compiled
    println!("cargo:rustc-link-search=native={lib_path}");
    println!("cargo:rustc-link-lib=static=FastPFOR");
}

/// Build script entry point.
fn main() {
    #[cfg(feature = "cpp")]
    build_fastpfor();
}
