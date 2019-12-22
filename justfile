test: 
    cargo watch --clear --exec test

check: 
    cargo watch --clear --exec check

test-print:
    cargo test -- --nocapture

build: 
	cargo build --release

run:
    cargo run -- 4 5 6 :mul .

publish:
	cargo build
	cargo publish
