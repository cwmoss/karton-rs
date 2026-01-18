all: target/release/karton

target/release/karton: src/main.rs
	cargo build --release
	cp target/release/karton /usr/local/bin/karton
	ls -alh ./target/release/karton
	ls -al ./target/release/karton

ubuntu.20.04:
	# todo: make this work
	docker run --rm --user "$(id -u)":"$(id -g)" -v "$PWD":/usr/src/myapp -w /usr/src/myapp devraymondsh/ubuntu-rust:20.04 cargo build --release