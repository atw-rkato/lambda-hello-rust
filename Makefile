build-HelloRustFunction:
	cargo build --release -j 12 --target x86_64-unknown-linux-musl
	cp ./target/x86_64-unknown-linux-musl/release/bootstrap $(ARTIFACTS_DIR)
