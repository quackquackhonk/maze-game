build:
	cargo build --release --manifest-path XGame/Cargo.toml
	cp XGame/target/release/xgames xgames
	cp XGame/target/release/xgames-with-observer xgames-with-observer

clean:
	cargo clean --manifest-path Xchoice/Cargo.toml
	rm xchoice

.PHONY: clean
