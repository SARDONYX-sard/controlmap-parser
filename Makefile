default: json

json:
	cargo run --example controlmap-parser .\examples\controlmap_sample.txt > result.json

controlmap:
	cargo run --example controlmap-parser .\examples\controlmap_sample.txt

test:
	cargo run --example controlmap-parser .\test-files\controlmap_test.txt > result.json
