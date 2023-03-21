.PHONY: build, lint, test

build:
	cargo build

lint:
	cargo fmt --all
	cargo clippy

test:
	cargo test
