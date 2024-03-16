linux-amd64-musl:
	rustup target add x86_64-unknown-linux-musl
	cargo build --release --target=x86_64-unknown-linux-musl

linux--amd64-gnu:
	#rustup target add x86_64-unknown-linux-gnu
	cargo build --release --target=x86_64-unknown-linux-gnu

linux-arm64-musl:
	#rustup target add aarch64-unknown-linux-musl
	cargo build --release --target=aarch64-unknown-linux-musl

linux-arm64-gnu:
	#rustup target add aarch64-unknown-linux-gnu
	cargo build --release --target=aarch64-unknown-linux-gnu

windows:
	#rustup target add x86_64-pc-windows-gnu
	cargo build --release --target=x86_64-pc-windows-gnu

macos-arm64:
	#rustup target add aarch64-apple-darwin
	cargo build --release --target=aarch64-apple-darwin

macos-x86_64:
	#rustup target add x86_64-apple-darwin
	cargo build --release --target=x86_64-apple-darwin
