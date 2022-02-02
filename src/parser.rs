use anyhow::Context;
use pest::{error::Error, iterators::Pairs, Parser};

#[derive(Parser, Debug)]
#[grammar = "pest/controlmap.pest"]
pub struct ControlMapParser;

pub fn parse(input: &str) -> Result<String, Error<ControlMapParser>> {
    // let successful_parse = ControlMapParser::parse(Rule::field, "Forward	0xff	0xff	0xff	0	0	0");
    let successful_parse = ControlMapParser::parse(Rule::rows, &input)
        .with_context(|| format!("unsuccessful parse"))
        .unwrap();

    fn parse_control_map(pairs: Pairs<Rule>) -> String {
        // println!("{}", &successful_parse);
        let mut string = String::new();

        let mut index = 0;
        pairs.for_each(|row| match row.as_rule() {
            Rule::event_name => {
                format!("----------------------------------------------------");
                string += &format!("event_name:    {}\n", &row.as_str())[..];
            }
            Rule::keycode => {
                string += &format!("   keycode:    {}\n", &row.as_str())[..];
            }
            Rule::blank_line => {
                string += &format!("   blank:    {}\n", &row.as_str())[..];
                index += 1;
                string += &format!("INDEX: {} \n", &index)[..];
            }
            Rule::comment => {
                string += &format!("   comment:    {}\n", &row.as_str())[..];
            }
            Rule::EOI => {
                string += &format!("   EOI:    {}\n", &row.as_str())[..];
            }
            _ => {
                format!("   Error:   {}", &row.as_str());
                unreachable!()
            }
        });

        string
    }

    Ok(parse_control_map(successful_parse))
}
