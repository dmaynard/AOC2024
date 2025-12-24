use miette::miette;
use nom::{
    IResult, Parser, bytes::complete::tag,
    character::complete, combinator::all_consuming,
    multi::separated_list1, sequence::separated_pair,
};
use rayon::prelude::*;
use std::ops::RangeInclusive;
use tracing::info;
pub fn process(input: &str) -> miette::Result<String> {
    let (_, id_ranges) =
        all_consuming(ranges).parse(input).map_err(
            |e| miette!("failed to parse aoc input, {e}"),
        )?;
    let total = id_ranges
        .into_par_iter()
        .map(|ids| {
            let mut total = 0;
            for id in ids.into_iter() {
                let id_str = id.to_string();
                let half = id_str.len() / 2;
                for limit in 0..half {
                    if id_str.len().rem_euclid(limit + 1)
                        == 0
                    {
                        let all_match = id_str[0..=limit]
                            .chars()
                            .cycle()
                            .zip(id_str.chars())
                            .all(|(a, b)| {
                                info!(?a, ?b);
                                a == b
                            });
                        if all_match {
                            info!(?id);
                            total += id;
                            if get_val(id) == 0{ println!("false negativeid: {}, val: {}", id, get_val(id));}
                           // break;
                        } else { if get_val(id) > 0 {println!("false positive id: {}, val: {}", id, get_val(id))};
                         //   break;
                        }
                    }
                }
            }
            total
        })
        .sum::<u64>();
    Ok(total.to_string())
}
#[tracing::instrument]
pub fn process_ilog(input: &str) -> miette::Result<String> {
    let mut id_max = 0;
    let (_, id_ranges) =
        all_consuming(ranges).parse(input).map_err(
            |e| miette!("failed to parse aoc input, {e}"),
        )?;
    let mut total = 0;
    for ids in id_ranges.into_iter() {
        for id in ids.into_iter() {
            if id > id_max {
                id_max = id;
            }
            total += get_val(id);
            // println!("id: {}, val: {}", id, get_val(id));
        }
    }
    println!("id_max: {}", id_max);
    Ok(total.to_string())
}

pub fn get_val (nid :u64) -> u64{
    let ilog = nid.ilog10();
    let mut val = match ilog {

        1 => if nid / 10 == nid % 10  {nid} else {0},
        2 => if (nid / 100 == (nid %100 )/ 10 ) && (nid / 100  == nid % 10 ) //  aaa
                 {nid} else {0}, 
        3 => if nid / 100 == nid % 100  {nid} else {0},
        5 => if nid / 1000 == nid % 1000  ||  //abc abc

                 (nid / 10000 == (nid %10000)/ 100 ) && (nid / 10000 == nid % 100 )    // ab ab ab 
                {nid} else {0},
                
        7 => if nid / 10000 == nid % 10000 {nid} else {0},
        8 => if 
        (nid / 1000000 == (nid %1000000)/ 1000 ) && (nid / 1000000 == nid % 1000 )    // abc abc abc 
       {nid} else {0},
  
        9 => if nid / 100000 == nid % 100000   ||  // abcde abcde 
         (
         (nid / 100000000 == nid %100) && 
         ((nid % 100000000/ 1000000) == nid % 100)  &&
         ((nid % 1000000/ 10000) == nid % 100)  &&
         ((nid % 10000/ 100) == nid % 100) 
         )

         {nid} else {0}, 
        11 => if nid / 1000000 == nid % 1000000  ||  //   abcdef abcdef
                   (nid / 100000000 == (nid %100000000)/ 10000 ) && (nid / 100000000 == nid % 10000)   // ab ab ab 
                {nid} else {0},
     
        
        
        _=> 0,
    };
    if all_digits_same(nid, ilog+1) {val += nid}
    val
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

fn all_digits_same(n: u64, num_digits: u32) -> bool {
    let last_digit = n % 10;
    (0..num_digits).all(|i| (n / 10_u64.pow(i)) % 10 == last_digit)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "123412341230-123412341238,11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        assert_eq!("4174379266", process(input)?);
        Ok(())
    }
}