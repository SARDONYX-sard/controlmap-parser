default: run

run:
	cargo run --example cast_and_fmt --features serde
	cargo run --example parse_and_print --features serde
	cargo run --example scan_code

doc:
	cargo doc --open

lint-fix:
	cargo fmt --all
	cargo clippy --fix --allow-staged --allow-dirty --features serde

test:
	cargo test --features serde

publish-test:
	cargo publish --dry-run;
	cargo package --list;

.PHONY: run doc test publish-test
