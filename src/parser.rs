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

    #[test]
    fn multiple_spaces() {
        assert_eq!(parse_message("a   b"), Ok(vec!("a", "b")));
    }

    #[test]
    fn symbols() {
        assert_eq!(parse_message("@t3$t_ t3st2"), Ok(vec!("@t3$t_", "t3st2")));
    }

    #[test]
    fn err_not_closed() {
        assert!(parse_message("a \"b").is_err());
    }

    #[test]
    fn err_not_opened() {
        assert!(parse_message("a b\"").is_err());
    }

    #[test]
    fn err_illegal_char() {
        assert!(parse_message("a \"b\nb\"").is_err());
    }
}