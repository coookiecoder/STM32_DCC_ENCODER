compile:
	cargo build --target thumbv7em-none-eabihf --release

add_target:
	rustup target add thumbv7em-none-eabihf
