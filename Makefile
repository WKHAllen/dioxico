all: build

build:
	cargo build

test:
	cargo test -- --nocapture

lint:
	cargo clippy -- -D warnings

demo:
	dx serve --package dioxico --example demo --platform desktop

demo-web:
	dx serve --package dioxico --example demo --platform web

clean:
	cargo clean
