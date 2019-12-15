test: 
    cargo watch --clear --exec test

check: 
    cargo watch --clear --exec check

test-print:
    cargo test -- --nocapture

build: 
	cargo build --release

publish:
	cargo build
	cargo publish
