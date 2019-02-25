use pest::Parser;

#[derive(Parser)]
#[grammar = "args_parse.pest"]
struct ArgsParser;

pub fn parse_message(msg: &str) -> Result<Vec<&str>, pest::error::Error<Rule>> {
    let pairs = ArgsParser::parse(Rule::line, msg)?;

    return Ok(pairs
        .filter(|pair| pair.as_rule() != Rule::EOI)
        .map(|pair| pair.as_str())
        .collect());
}
