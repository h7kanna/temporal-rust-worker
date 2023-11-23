NAME := "temporal-rust-worker"
ORG := "ekalavya-dev"
VERSION := `git rev-parse HEAD`
SEMVER_VERSION := `grep version Cargo.toml | awk -F"\"" '{print $2}' | head -n 1`

default:
  @just --list --unsorted --color=always | rg -v "    default"

check:
  @cargo check

fmt-check:
  @cargo +nightly fmt --check

fmt:
  @cargo +nightly fmt

lint:
  @cargo clippy --all-targets --all-features -- --no-deps -D warnings

lint-fix:
  @cargo clippy --all-targets --all-features --fix --allow-dirty --allow-staged -- --no-deps -D warnings

lint-pedantic:
  @cargo clippy --all-targets --all-features -- --no-deps -W clippy::pedantic -D warnings

lint-pedantic-fix:
  @cargo clippy --all-targets --all-features --fix --allow-dirty --allow-staged -- --no-deps -W clippy::pedantic -D warnings

test:
  @cargo test --all-targets --all-features
