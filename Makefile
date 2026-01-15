all: bin/karton

bin/karton: src/main.rs
	cargo build --release
	cp target/release/karton-rs bin/karton