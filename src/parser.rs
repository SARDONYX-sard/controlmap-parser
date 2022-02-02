use anyhow::Context;
use indexmap::IndexMap;
use pest::{error::Error, iterators::Pairs, Parser};

#[derive(Parser, Debug)]
#[grammar = "pest/controlmap.pest"]
pub struct ControlMapParser;

/// Parses a given controlmap.txt file at the given file path.
///
/// # Examples
///
/// ```
/// use controlmap_parser::parse;
///
/// let unparsed_file = std::fs::read_to_string("test-files/controlmap_test.txt").expect("Failed to read file.");
/// let parsed = parse(&unparsed_file).expect("Failed to parse file.");
/// let result = std::fs::read_to_string("test-files/expected.json")
///     .expect("Failed to read expected file.");
/// assert_eq!(parsed, result);
///```
pub fn parse(input: &str) -> Result<String, Error<ControlMapParser>> {
    let successful_parse = ControlMapParser::parse(Rule::rows, &input)
        .with_context(|| format!("unsuccessful parse"))
        .unwrap();

    Ok(parse_ctrlmap(successful_parse))
}

fn parse_ctrlmap(pairs: Pairs<Rule>) -> String {
    let ctrlmap_category = vec![
        "Main Gameplay",
        "Menu Mode",
        "Console",
        "Item Menus",
        "Inventory",
        "Debug Text",
        "Map Menu",
        "Stats",
        "Cursor",
        "Book",
        "Debug overlay",
        "Journal",
        "TFC mode",
        "Debug Map Menu-like mode (but not the actual map menu)",
        "Lockpicking",
        "Favor",
    ];

    let mut event_count = 0;
    let mut index: usize = 0;
    let mut event_name = String::new();
    let mut event = IndexMap::new();
    let mut events = IndexMap::new();
    let mut comment = vec![];
    let mut keycode_list = vec![];

    pairs.for_each(|row| match row.as_rule() {
        Rule::event_name => {
            if event_count == 0 {
            } else {
                event.insert("comment".to_string(), comment.clone());
                event.insert(event_name.clone(), keycode_list.clone());
                keycode_list = vec![];
            }

            event_name = row.as_str().to_string();
            event_count += 1;
        }
        Rule::comment => comment.push(row.as_str()),
        Rule::keycode => keycode_list.push(row.as_str()),
        Rule::blank_line => {
            events.insert(ctrlmap_category[index], event.clone());

            event = IndexMap::new();
            comment = vec![];
            index += 1;
        }
        Rule::EOI => {
            format!("END:   {}", &row.as_str());
        }
        _ => unreachable!(),
    });
    format!("{:?}", events.clone())[..].to_string()
}
