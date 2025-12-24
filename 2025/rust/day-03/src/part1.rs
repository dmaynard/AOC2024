
use itertools::Itertools;
use tracing::info;

#[tracing::instrument]
pub fn process_a(input: &str) -> miette::Result<String> {
    let result = input
        .lines()
        .map(|bank| {
            let (index, first_max) = &bank
                [..(bank.len() - 1)]
                .chars()
                .enumerate()
                .max_set_by_key(|(_index, battery)| {
                    *battery
                })
                .first()
                .cloned()
                .unwrap();

            debug_assert!(!bank[(index + 1)..].is_empty());

            let (_second_index, second_max) = &bank
                [(index + 1)..]
                .chars()
                .enumerate()
                .max_by_key(|(_index, battery)| *battery)
                .unwrap();

            format!("{first_max}{second_max}")
                .parse::<u64>()
                .unwrap()
        })
        .sum::<u64>();

    Ok(result.to_string())
}
pub fn process_b(input: &str) -> miette::Result<String> {
    let result  = input
        .lines()
        .map(|bank| {
            let n_banks = 2;
            let mut batteries: Vec<char> = vec![];
            let (mut left , mut right) = (0, bank.len() - n_banks);
            let bank_map = sorted_indexed_chars(bank);
            for _i in 0..n_banks {
               //  dbg!(bank_map.clone());
               //  dbg!(left, right);
                let (index, first_max) = bank_map
                    .iter()
                    .filter(|(idx, _)| *idx >= left && *idx <= right)
                .next().unwrap();
               // dbg!( index, first_max);
                batteries.push(*first_max);
                left = index + 1;
                right = right + 1;
               
            }
            batteries
            .iter()
            .collect::<String>()
            .parse::<u64>()
            .unwrap()
        })
        .sum::<u64>();
       
    Ok(result.to_string())
}
fn sorted_indexed_chars(s: &str) -> Vec<(usize, char)> {
    let mut indexed: Vec<_> = s.chars().enumerate().map(|(i, c)| (i, c)).collect();
    indexed.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    indexed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let input = "987654321111111
811111111111119
234234234234278
818181911112111";
        // let input = "897777777789777";
        assert_eq!("357", process_a(input)?);
        assert_eq!("357", process_b(input)?);
        Ok(())
    }
}