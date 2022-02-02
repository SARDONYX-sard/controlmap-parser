default: controlmap

json:
	cargo run --example json -- .\examples\sample.json

controlmap:
	cargo run -- .\examples\controlmap_sample2.txt
