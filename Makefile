all: build

build:
	cargo build

test:
	cargo test -- --nocapture

lint:
	cargo clippy -- -D warnings

demo:
	dx serve --hot-reload --example demo

clean:
	cargo clean
