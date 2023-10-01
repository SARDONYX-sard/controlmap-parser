default: run

run:
	cargo run --example parse_to_json --features serde > result.json

doc:
	cargo doc --open

test:
	cargo test --features serde

publish-test:
	cargo publish --dry-run;
	cargo package --list;

.PHONY: run doc test publish-test
