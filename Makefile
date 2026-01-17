all: target/release/karton

target/release/karton: src/main.rs
	cargo build --release
	cp target/release/karton /usr/local/bin/karton
	ls -alh ./target/release/karton
	ls -al ./target/release/karton