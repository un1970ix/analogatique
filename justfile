set shell := ["sh", "-eu", "-c"]
set positional-arguments

[doc("List the available recipes.")]
default:
    @just --list

[doc("Fetch the locked dependencies and install cargo-audit.")]
setup:
    cargo fetch --locked
    cargo install cargo-audit --locked

[doc("Format the Rust source code.")]
fmt:
    cargo fmt

[doc("Check Rust formatting and lints.")]
check:
    cargo fmt --check
    cargo clippy --locked --all-targets -- -D warnings

[doc("Run the Rust tests.")]
test *args:
    cargo test --locked "$@"

[doc("Build the release binary.")]
build:
    cargo build --locked --release

[doc("Run the gallery generator.")]
run *args:
    cargo run --locked -- "$@"

[doc("Audit dependencies for vulnerabilities.")]
audit:
    cargo audit

[doc("Report audit failures without blocking later commands.")]
audit-report:
    just audit || printf '%s\n' 'Warning: Dependency audit failed; continuing.' >&2

[doc("Validate the release without publishing it.")]
deploy-check: check test build audit-report
