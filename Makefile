run: build
	cargo run -j 6 --release

build:
	cargo build -j 6 --release

test:
	cargo test -j 6