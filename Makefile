.PHONY: example

example:
	cargo run --features example

clean:
	cargo clippy --fix --allow-dirty --allow-staged
	cargo fmt
