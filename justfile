#!/usr/bin/env just --justfile

main_crate := 'fastpfor'

# if running in CI, treat warnings as errors by setting RUSTFLAGS and RUSTDOCFLAGS to '-D warnings' unless they are already set
# Use `CI=true just ci-test` to run the same tests as in GitHub CI.
# Use `just env-info` to see the current values of RUSTFLAGS and RUSTDOCFLAGS
export RUSTFLAGS := env('RUSTFLAGS', if env('CI', '') == 'true' {'-D warnings'} else {''})
export RUSTDOCFLAGS := env('RUSTDOCFLAGS', if env('CI', '') == 'true' {'-D warnings'} else {''})
export RUST_BACKTRACE := env('RUST_BACKTRACE', if env('CI', '') == 'true' {'1'} else {''})

@_default:
    just --list

# Run integration tests and save its output as the new expected output
bless *args:  (cargo-install 'cargo-insta')
    TRYBUILD=overwrite cargo insta test --accept --unreferenced=delete --all-features

# Build the project
build:
    cargo build --workspace --all-targets --all-features

# Quick compile without building a binary
check:
    cargo check --workspace --all-targets --all-features

# Verify that the current version of the crate is not the same as the one published on crates.io
check-if-published:  (assert 'jq')
    #!/usr/bin/env bash
    set -euo pipefail
    LOCAL_VERSION="$({{just_executable()}} get-crate-field version)"
    echo "Detected crate version:  '$LOCAL_VERSION'"
    CRATE_NAME="$({{just_executable()}} get-crate-field name)"
    echo "Detected crate name:     '$CRATE_NAME'"
    PUBLISHED_VERSION="$(cargo search ${CRATE_NAME} | grep "^${CRATE_NAME} =" | sed -E 's/.* = "(.*)".*/\1/')"
    echo "Published crate version: '$PUBLISHED_VERSION'"
    if [ "$LOCAL_VERSION" = "$PUBLISHED_VERSION" ]; then
        echo "ERROR: The current crate version has already been published."
        exit 1
    else
        echo "The current crate version has not yet been published."
    fi

# Generate code coverage report to upload to codecov.io
ci-coverage: && \
            (coverage '--codecov --output-path target/llvm-cov/codecov.info')
    # ATTENTION: the full file path above is used in the CI workflow
    mkdir -p target/llvm-cov

# Run all tests as expected by CI
ci-test: env-info test-fmt build clippy test test-doc
    #!/usr/bin/env bash
    set -euo pipefail
    if [ -n "$(git status --untracked-files --porcelain)" ]; then
      >&2 echo 'ERROR: git repo is no longer clean. Make sure compilation and tests artifacts are in the .gitignore, and no repo files are modified.'
      >&2 echo '######### git status ##########'
      git status
      exit 1
    fi

# Run minimal subset of tests to ensure compatibility with MSRV
ci-test-msrv: env-info test

# Clean all build artifacts
clean:
    cargo clean
    rm -f Cargo.lock

# Run cargo clippy to lint the code
clippy *args:
    cargo clippy --workspace --all-targets --all-features {{args}}

# Generate code coverage report. Will install `cargo llvm-cov` if missing.
coverage *args='--no-clean --open':  (cargo-install 'cargo-llvm-cov')
    cargo llvm-cov --workspace --all-targets --all-features --include-build-script {{args}}

# Build and open code documentation
docs:
    cargo doc --no-deps --all-features --open

# Print environment info
env-info:
    @echo "Running on {{os()}} / {{arch()}}"
    {{just_executable()}} --version
    rustc --version
    cargo --version
    rustup --version
    @echo "RUSTFLAGS='$RUSTFLAGS'"
    @echo "RUSTDOCFLAGS='$RUSTDOCFLAGS'"

# Reformat all code `cargo fmt`. If nightly is available, use it for better results
fmt:
    #!/usr/bin/env bash
    set -euo pipefail
    if rustup component list --toolchain nightly | grep rustfmt &> /dev/null; then
        echo 'Reformatting Rust code using nightly Rust fmt to sort imports'
        cargo +nightly fmt --all -- --config imports_granularity=Module,group_imports=StdExternalCrate
    else
        echo 'Reformatting Rust with the stable cargo fmt.  Install nightly with `rustup install nightly` for better results'
        cargo fmt --all
    fi

# Get any package's field from the metadata
get-crate-field field package=main_crate:
    cargo metadata --format-version 1 | jq -r '.packages | map(select(.name == "{{package}}")) | first | .{{field}}'

# Get the minimum supported Rust version (MSRV) for the crate
get-msrv:  (get-crate-field 'rust_version')

# Find the minimum supported Rust version (MSRV) using cargo-msrv extension, and update Cargo.toml
msrv:  (cargo-install 'cargo-msrv')
    cargo msrv find --write-msrv --component rustfmt --all-features --ignore-lockfile -- {{just_executable()}} ci-test-msrv

# Check semver compatibility with prior published version. Install it with `cargo install cargo-semver-checks`
semver *args:  (cargo-install 'cargo-semver-checks')
    cargo semver-checks {{args}}

# Run all tests
test:
    cargo test --workspace --all-targets --all-features

# Test documentation
test-doc:
    cargo test --doc --all-features
    cargo doc --all-features --no-deps

# Test code formatting
test-fmt:
    cargo fmt --all -- --check

# Use the experimental workspace publishing with --dry-run. Requires nightly Rust.
test-publish:
    cargo +nightly -Z package-workspace publish --dry-run

# Find unused dependencies. Install it with `cargo install cargo-udeps`
udeps:  (cargo-install 'cargo-udeps')
    cargo +nightly udeps --all-targets --workspace --all-features

# Update all dependencies, including breaking changes. Requires nightly toolchain (install with `rustup install nightly`)
update:
    cargo +nightly -Z unstable-options update --breaking
    cargo update

# Ensure that a certain command is available
[private]
assert command:
    @if ! type {{command}} > /dev/null; then \
        echo "Command '{{command}}' could not be found. Please make sure it has been installed on your computer." ;\
        exit 1 ;\
    fi

# Check if a certain Cargo command is installed, and install it if needed
[private]
cargo-install $COMMAND $INSTALL_CMD='' *args='':
    #!/usr/bin/env bash
    set -euo pipefail
    if ! command -v $COMMAND > /dev/null; then
        if ! command -v cargo-binstall > /dev/null; then
            echo "$COMMAND could not be found. Installing it with    cargo install ${INSTALL_CMD:-$COMMAND} --locked {{args}}"
            cargo install ${INSTALL_CMD:-$COMMAND} --locked {{args}}
        else
            echo "$COMMAND could not be found. Installing it with    cargo binstall ${INSTALL_CMD:-$COMMAND} --locked {{args}}"
            cargo binstall ${INSTALL_CMD:-$COMMAND} --locked {{args}}
        fi
    fi
