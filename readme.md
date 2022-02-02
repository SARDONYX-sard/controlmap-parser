# ControlMap Parser Library

![controlmap sample](./docs/sample-code.png)

## Abstract

This is the controlmap.txt parser.

controlmap.txt => json file

## Requirements

- git
- Rust

## Getting Started

```bash
git clone ;
cd controlmap-parser;

make; # sample controlmap.txt to result.json
# or
make controlmap; # controlmap.txt to stdout (show display)

# or manual
cargo run --example controlmap-parser <your controlmap.txt file path>
```

## License

[MIT](https://opensource.org/licenses/MIT)
