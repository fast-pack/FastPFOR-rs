#!/usr/bin/env just --justfile

@_default:
    just --list

# Clean all build artifacts
clean:
    cargo clean
    rm -f Cargo.lock

# Update dependencies, including breaking changes
update:
    cargo +nightly -Z unstable-options update --breaking
    cargo update

# Run cargo clippy
clippy:
    cargo clippy --all-targets --workspace -- -D warnings

# Test code formatting
test-fmt:
    cargo fmt --all -- --check

# Run cargo fmt
fmt:
    cargo +nightly fmt -- --config imports_granularity=Module,group_imports=StdExternalCrate

# Build and open code documentation
docs:
    cargo doc --no-deps --open

# Quick compile
check:
    cargo check --all-targets --workspace

# Default build
build *ARGS:
    cargo build --all-targets --workspace {{ARGS}}

# Run all tests
test *ARGS: build
    cargo test --all-targets --workspace {{ARGS}}

# Find the minimum supported Rust version. Install it with `cargo install cargo-msrv`
msrv:
    cargo msrv find --component rustfmt -- {{just_executable()}} ci-test-msrv

# Find unused dependencies. Install it with `cargo install cargo-udeps`
udeps:
    cargo +nightly udeps --all-targets --workspace

# Check semver compatibility with prior published version. Install it with `cargo install cargo-semver-checks`
semver *ARGS:
    cargo semver-checks {{ARGS}}

# Generate and show coverage report. Requires grcov to be installed.
grcov:
    #!/usr/bin/env bash
    set -euo pipefail
    find . -name '*.profraw' | xargs rm
    rm -rf ./target/debug/coverage
    export LLVM_PROFILE_FILE="fastpfor-%p-%m.profraw"
    export RUSTFLAGS="-Cinstrument-coverage"
    cargo build --all-targets --workspace
    cargo test --all-targets --workspace
    grcov . -s . --binary-path ./target/debug/ -t html --branch --ignore-not-existing -o ./target/debug/coverage/
    open ./target/debug/coverage/index.html

# Use the experimental workspace publishing with --dry-run. Requires nightly Rust.
test-publish:
    cargo +nightly -Z package-workspace publish --dry-run

# Run tests, and accept their results. Requires insta to be installed.
bless:
    TRYBUILD=overwrite cargo insta test --accept

# Test documentation
test-doc:
    cargo test --doc
    cargo doc --no-deps

rust-info:
    rustc --version
    cargo --version

# Run tests only relevant to the latest version
ci-test-extras: test-doc

# Run all tests as expected by CI
ci-test: rust-info test-fmt clippy test build

# Run minimal subset of tests to ensure compatibility with MSRV
ci-test-msrv: rust-info test

# Verify that the current version of the crate is not the same as the one published on crates.io
check-if-published:
    #!/usr/bin/env bash
    set -euo pipefail
    LOCAL_VERSION="$(grep '^version =' Cargo.toml | sed -E 's/version = "([^"]*)".*/\1/')"
    echo "Detected crate version:  $LOCAL_VERSION"
    CRATE_NAME="$(grep '^name =' Cargo.toml | head -1 | sed -E 's/name = "(.*)"/\1/')"
    echo "Detected crate name:     $CRATE_NAME"
    PUBLISHED_VERSION="$(cargo search ${CRATE_NAME} | grep "^${CRATE_NAME} =" | sed -E 's/.* = "(.*)".*/\1/')"
    echo "Published crate version: $PUBLISHED_VERSION"
    if [ "$LOCAL_VERSION" = "$PUBLISHED_VERSION" ]; then
        echo "ERROR: The current crate version has already been published."
        exit 1
    else
        echo "The current crate version has not yet been published."
    fi
