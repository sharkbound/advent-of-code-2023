use nom::branch::alt;
use nom::character::complete::{line_ending, multispace1};
use nom::IResult;
use nom::multi::many0;

/**
consumes whitespace and newlines until there is no more to consume.
 */
pub fn consume_empty_space(input: &str) -> IResult<&str, ()> {
    let (input, _) = many0(alt((multispace1, line_ending)))(input)?;
    Ok((input, ()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_consumes_preceeding_spaces() {
        let input = "  \n\n\r\n   abc";
        let (input, _) = consume_empty_space(input).expect("");
        assert_eq!(input, "abc");
    }
}