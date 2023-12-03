use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::IResult;
use nom::sequence::tuple;

fn nom(input: &str) -> IResult<&str, &str> {
    Ok(alt((tag("abc"), tag("def"), tag("ghi")))(input)?)
}
fn main() {
    println!("{:?}", nom("def"));
}