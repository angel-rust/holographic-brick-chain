default: build

build:
	cargo build --workspace

test:
	cargo test --workspace

fmt:
	cargo fmt --all

lint:
	cargo clippy --workspace -- -D warnings

run-cli args="hello":
	cargo run -p brick-cli -- {{args}}
