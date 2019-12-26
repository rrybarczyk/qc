test: 
    cargo watch --clear --exec test

check: 
    cargo watch --clear --exec check

test-print:
    cargo test -- --nocapture

build: 
	cargo build --release

run:
    cargo watch --clear --exec run --shell "./target/debug/qc 2 3 4 :add . && ./target/debug/qc 4 7 9 :add 2 3 5 :mul 1 1 :sub 20 5 :div ."

publish:
	cargo build
	cargo publish
