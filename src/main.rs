fn main() {
    let path = std::env::args().nth(1).expect("Failed to parse arguments.");
    let unparsed_file = std::fs::read_to_string(path).expect("Failed to read file.");

    let parsed = controlmap_parser::parse(&unparsed_file).expect("Failed to parse file.");
    println!("{}", parsed);
}
