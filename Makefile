all:
	mkdir ./package || true
	cargo fmt
	cargo build --release
	cp target/release/satr ./package

dev:
	mkdir ./package || true
	cargo build
	cp target/debug/satr ./package


install:
	cp ./package/satr /bin
