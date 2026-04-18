all: build

build:
	cargo build

test:
	cargo test -- --nocapture

lint:
	cargo clippy -- -D warnings

demo:
	dx serve --example demo --platform desktop

demo-web:
	dx serve --example demo --platform web

clean:
	cargo clean
