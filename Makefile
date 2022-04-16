build: src
	cargo build --release

lint:
	cargo clippy

test:
	cargo test -- --nocapture
