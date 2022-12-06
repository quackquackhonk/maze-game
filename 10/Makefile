build:
	cargo build --release --bin server --bin client
	cp ../target/release/server xserver
	cp ../target/release/client xclients

clean:
	cargo clean 
	rm xserver
	rm xclients

.PHONY: clean
