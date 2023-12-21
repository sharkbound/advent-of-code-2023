use nom::branch::alt;
use nom::character::complete::{multispace1, newline};
use nom::combinator::map_res;
use nom::IResult;
use nom::multi::many0;

/**
 consumes whitespace and newlines until there is no more to consume.
 */
pub fn consume_empty_space(input: &str) -> IResult<&str, ()> {
    let (input, _) = many0(alt((multispace1, map_res(newline, |_c: char| Result::<&str, ()>::Ok("")))))(input)?;
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