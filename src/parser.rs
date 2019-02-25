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

#[cfg(test)]
mod tests {
    use crate::parser::parse_message;

    #[test]
    fn simple() {
        assert_eq!(parse_message("a"), Ok(vec!("a")));
    }

    #[test]
    fn simple_quoted() {
        assert_eq!(parse_message("\"a\""), Ok(vec!("a")));
    }

    #[test]
    fn two() {
        assert_eq!(parse_message("a b"), Ok(vec!("a", "b")));
    }

    #[test]
    fn two_quote_1() {
        assert_eq!(parse_message("\"a\" b"), Ok(vec!("a", "b")));
    }

    #[test]
    fn two_quote_2() {
        assert_eq!(parse_message("a \"b\""), Ok(vec!("a", "b")));
    }

    #[test]
    fn many() {
        assert_eq!(parse_message("a bb \"c c\" ddd \"e\" f"),
                   Ok(vec!("a", "bb", "c c", "ddd", "e", "f")));
    }
}