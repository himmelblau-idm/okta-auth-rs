.DEFAULT_GOAL := build

.PHONY: help build build-mfa-example run-mfa-example release test test-hsm parity generate check-generated inspect source-hash fmt fmt-check clean publish publish-dry-run

help: ## Show available maintenance commands
	@printf 'Usage: make [target]\n\n'
	@awk 'BEGIN {FS = ":.*## "} /^[A-Za-z0-9_-]+:.*## / {printf "  %-20s %s\n", $$1, $$2}' $(MAKEFILE_LIST)
	@printf '\nRequirements: Cargo and the pinned Rust toolchain with rustfmt.\n'
	@printf 'Parity also requires Python 3, Node.js, and npm.\n'
	@printf 'Publishing requires registry credentials; version changes and tags are manual.\n'

build: ## Build the workspace in the development profile
	cargo build --locked --workspace

build-mfa-example: ## Build the interactive MFA example with proxyable support
	cargo build --locked -p okta-auth-rs --features proxyable --example okta-mfa

run-mfa-example: export ISSUER := $(ISSUER)
run-mfa-example: export CLIENT_ID := $(CLIENT_ID)
run-mfa-example: export REDIRECT_URI := $(REDIRECT_URI)
run-mfa-example: build-mfa-example ## Run MFA example (ISSUER=... CLIENT_ID=... REDIRECT_URI=...)
	@if [ -z "$$ISSUER" ] || [ -z "$$CLIENT_ID" ] || [ -z "$$REDIRECT_URI" ]; then \
		printf '%s\n' 'Error: ISSUER, CLIENT_ID, and REDIRECT_URI are all required.' \
			'Usage: make run-mfa-example ISSUER=https://YOUR_ORG.okta.com/oauth2/default CLIENT_ID=YOUR_CLIENT_ID REDIRECT_URI=http://127.0.0.1:8765/callback' >&2; \
		exit 1; \
	fi
	cargo run --locked -p okta-auth-rs --features proxyable --example okta-mfa -- "$$ISSUER" "$$CLIENT_ID" "$$REDIRECT_URI"

release: ## Build the workspace in the release profile
	cargo build --locked --workspace --release

test: check-generated fmt-check ## Run workspace tests with default features
	cargo test --locked --workspace --all-targets

test-hsm: check-generated fmt-check ## Run library tests with HSM support
	cargo test --locked -p okta-auth-rs --features hsm

parity: check-generated fmt-check ## Install oracle dependencies and compare upstream/Rust behavior
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
