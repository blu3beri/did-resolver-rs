build: 
	cargo build --release

lint:
	cargo clippy

test:
	cargo test -- --nocapture
