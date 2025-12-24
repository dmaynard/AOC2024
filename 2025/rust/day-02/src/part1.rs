use miette::miette;
use nom::{
    IResult, Parser, bytes::complete::tag,
    character::complete, combinator::all_consuming,
    multi::separated_list1, sequence::separated_pair,
};
use std::ops::RangeInclusive;
use tracing::info;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let (_, id_ranges) =
        all_consuming(ranges).parse(input).map_err(
            |e| miette!("failed to parse aoc input, {e}"),
        )?;
    let mut total = 0;
    for ids in id_ranges.into_iter() {
        for id in ids.into_iter() {
            total += getVal(id);
           //  println!("id: {}, val: {}", id, getVal(id));
        }
    }
    Ok(total.to_string())
}

pub fn getVal (nid :u64) -> u64{
    let ilog = nid.ilog10();
    match ilog {

        1 => if nid / 10 == nid % 10  {nid} else {0},
        3 => if nid / 100 == nid % 100  {nid} else {0},
        5 => if nid / 1000 == nid % 1000 {nid} else {0},
        7 => if nid / 10000 == nid % 10000 {nid} else {0},
        9 => if nid / 100000 == nid % 100000 {nid} else {0},
        11 => if nid / 1000000 == nid % 1000000 {nid} else {0},
        _=> 0,
    }
}

pub fn ranges(
    input: &str,
) -> IResult<&str, Vec<RangeInclusive<u64>>> {
    separated_list1(
        tag(","),
        separated_pair(
            complete::u64,
            tag("-"),
            complete::u64,
        )
        .map(|(start, end)| start..=end),
    )
    .parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        assert_eq!("1227775554", process(input)?);
        Ok(())
    }
}