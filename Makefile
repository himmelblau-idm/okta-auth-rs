.DEFAULT_GOAL := build

.PHONY: help build release test test-hsm parity generate check-generated inspect source-hash fmt fmt-check clean publish publish-dry-run

help: ## Show available maintenance commands
	@printf 'Usage: make [target]\n\n'
	@awk 'BEGIN {FS = ":.*## "} /^[A-Za-z0-9_-]+:.*## / {printf "  %-20s %s\n", $$1, $$2}' $(MAKEFILE_LIST)
	@printf '\nRequirements: Cargo and the pinned Rust toolchain with rustfmt.\n'
	@printf 'Parity also requires Python 3, Node.js, and npm.\n'
	@printf 'Publishing requires registry credentials; version changes and tags are manual.\n'

build: ## Build the workspace in the development profile
	cargo build --locked --workspace

release: ## Build the workspace in the release profile
	cargo build --locked --workspace --release

test: ## Run workspace tests with default features
	cargo test --locked --workspace --all-targets

test-hsm: ## Run library tests with HSM support
	cargo test --locked -p okta-auth-rs --features hsm

parity: ## Install oracle dependencies and compare upstream/Rust behavior
	npm ci --prefix tests/oracle
	cargo build --locked -p okta-auth-rs --example parity --target-dir target
	python3 tests/parity.py

generate: ## Regenerate Rust code and provenance from vendored Auth JS
	cargo run --locked -p okta-translator -- generate

check-generated: ## Verify generated code and provenance are reproducible
	cargo run --locked -p okta-translator -- check

inspect: ## Print the selected upstream graph and native substitutions
	cargo run --locked -p okta-translator -- inspect

source-hash: ## Print the deterministic vendored source tree hash
	cargo run --locked -p okta-translator -- source-hash

fmt: ## Format Rust source
	cargo fmt --all

fmt-check: ## Check Rust formatting without changing files
	cargo fmt --all -- --check

clean: ## Remove Cargo build artifacts
	cargo clean

publish: ## Publish okta-auth-rs to crates.io
	cargo publish --locked -p okta-auth-rs

publish-dry-run: ## Verify crate packaging without uploading
	cargo publish --locked -p okta-auth-rs --dry-run
