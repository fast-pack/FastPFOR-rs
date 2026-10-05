//! Build script for `FastPFOR-rs`.

/// Builds the C++ `FastPFOR` library and bridge when the `cpp` feature is enabled.
#[cfg(feature = "cpp")]
fn build_fastpfor() {
    use std::env;
    use std::path::Path;

    // docs.rs builds with all features, but documentation is only generated, never linked, so the
    // slow C++ build can be skipped.
    println!("cargo:rerun-if-env-changed=DOCS_RS");
    if env::var_os("DOCS_RS").is_some() {
        return;
    }

    assert!(
        Path::new("cpp/CMakeLists.txt").exists(),
        "FastPFOR submodule not initialized. Run `git submodule update --init`."
    );

    // Compile FastPFOR using CMake
    println!("cargo:rerun-if-changed=cpp");

    // Warn if more than one feature is enabled. The order is important, must match the if else block below.
    let simd_features = [
        (cfg!(feature = "cpp_portable"), "'cpp_portable'"),
        (cfg!(feature = "cpp_native"), "'cpp_native'"),
    ];
    let enabled_simd_features: Vec<_> = simd_features
        .into_iter()
        .filter_map(|(enabled, name)| enabled.then_some(name))
        .collect();

    // SIMD mode configuration via environment variable:
    // - native: Use -march=native for maximum performance (not portable across CPUs)
    // - portable: Use baseline SSE4.2 only for maximum compatibility (default)
    let simd_mode = env::var("FASTPFOR_SIMD_MODE");
    if enabled_simd_features.len() > 1 {
        let feats = enabled_simd_features.join(", ");
        if let Ok(simd_mode) = &simd_mode {
            println!(
                "cargo::warning=Multiple SIMD mode features are enabled: {feats}, but FASTPFOR_SIMD_MODE overrides it with {simd_mode}."
            );
        } else {
            println!(
                "cargo::warning=Multiple SIMD mode features enabled: {feats}. Defaulting to {}.",
                enabled_simd_features[0]
            );
        }
    }

    let simd_mode = simd_mode.as_deref().unwrap_or({
        {
            // The order is important, must match the list above.
            if cfg!(feature = "cpp_portable") {
                "portable"
            } else if cfg!(feature = "cpp_native") {
                "native"
            } else {
                "portable" // fallback
            }
        }
    });
    println!("cargo:rerun-if-env-changed=FASTPFOR_SIMD_MODE");

    // The C++ CMake build adds `-msse4.2` in portable mode unconditionally,
    // which non-x86 compilers reject. Fall back to native there.
    let is_x86 = env::var("CARGO_CFG_TARGET_ARCH").is_ok_and(|arch| arch.starts_with("x86"));
    let simd_mode = if !is_x86 && simd_mode == "portable" {
        println!(
            "cargo::warning=FASTPFOR_SIMD_MODE=portable is x86-only in the C++ library; using native."
        );
        "native"
    } else {
        simd_mode
    };

    let cmake_out = cmake::Config::new("cpp")
        .define("FASTPFOR_WITH_TEST", "OFF")
        .define("FASTPFOR_SIMD_MODE", simd_mode)
        .build();
    let lib_path = cmake_out.join("lib");
    let lib_path = lib_path.to_str().expect("path is not valid utf-8");

    // Compile the bridge
    println!("cargo:rerun-if-changed=src/cpp/fastpfor_bridge.h");
    println!("cargo:rerun-if-changed=src/cpp/ffi.rs");
    cxx_build::bridge("src/cpp/ffi.rs")
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
