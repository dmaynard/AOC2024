use miette::miette;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, anychar},
    combinator::value,
    multi::{many1, many_till},
    sequence::{delimited, separated_pair},
    IResult, Parser,
};
use tracing::info;
#[derive(Copy, Clone, Debug)]
enum InstructionSet {
    Mul(u32, u32),
    Do,
    Dont,
}
// #[tracing::instrument]
// #[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let (_input, instructions) = parse(input).map_err(|e| miette!("parse failed {}", e))?;
    info!("processing input ");
    // dbg!(&instructions);
    let mut scale: u32 = 1;
    let result: u32 = instructions
        .iter()
        .map(|ins| match ins {
            InstructionSet::Mul(a, b) => a * b * scale,
            InstructionSet::Do => {
                scale = 1;
                0
            }
            InstructionSet::Dont => {
                scale = 0;
                0
            }
        })
        .sum();
    Ok(result.to_string())
}

fn mul(input: &str) -> IResult<&str, InstructionSet> {
    let (input, _) = tag("mul")(input)?;
    let (input, pair) = delimited(
        tag("("),
        separated_pair(complete::u32, tag(","), complete::u32),
        tag(")"),
    )(input)?;
    Ok((input, InstructionSet::Mul(pair.0, pair.1)))
}
fn instruction(input: &str) -> IResult<&str, InstructionSet> {
    alt((
        value(InstructionSet::Dont, tag("don't()")),
        value(InstructionSet::Do, tag("do()")),
        mul,
    ))(input)
}
fn parse(input: &str) -> IResult<&str, Vec<InstructionSet>> {
    many1(many_till(anychar, instruction).map(|(_discard, ins)| ins))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!("48", process(input)?);
        Ok(())
    }
}
