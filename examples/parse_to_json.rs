use controlmap_parser::control_map_parser;
use nom::error::convert_error;

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let input = include_str!("./controlmap.txt");

    match control_map_parser(input) {
        Ok((_input, event_lines)) => {
            println!("{}", serde_json::to_string_pretty(&event_lines).unwrap());
        }
        Err(err) => match err {
            nom::Err::Incomplete(_) => println!("Parsing error: {:?}", err),
            nom::Err::Error(err) => {
                println!("Parsing error: {:?}", convert_error(input, err))
            }
            nom::Err::Failure(err) => {
                println!("Parsing error: {:?}", convert_error(input, err))
            }
        },
    }
    Ok(())
}
