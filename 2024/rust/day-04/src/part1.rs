use glam::IVec2;
use std::collections::HashMap;
use tracing::info;

const DIRECTIONS: [[IVec2; 3]; 8] = [
    [IVec2::new(0, 1), IVec2::new(0, 2), IVec2::new(0, 3)],
    [IVec2::new(0, -1), IVec2::new(0, -2), IVec2::new(0, -3)],
    [IVec2::new(1, 1), IVec2::new(2, 2), IVec2::new(3, 3)],
    [IVec2::new(1, -1), IVec2::new(2, -2), IVec2::new(3, -3)],
    [IVec2::new(-1, 1), IVec2::new(-2, 2), IVec2::new(-3, 3)],
    [IVec2::new(-1, -1), IVec2::new(-2, -2), IVec2::new(-3, -3)],
    [IVec2::new(1, 0), IVec2::new(2, 0), IVec2::new(3, 0)],
    [IVec2::new(-1, 0), IVec2::new(-2, 0), IVec2::new(-3, 0)],
];

#[tracing::instrument(skip(input))]
pub fn orig_process(input: &str) -> miette::Result<String> {
    let positions = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, value)| (IVec2::new(x as i32, y as i32), value))
        })
        .collect::<HashMap<IVec2, char>>();

    let mas = ['M', 'A', 'S'];
    let result: usize = positions
        .iter()
        .filter(|(_position, value)| **value == 'X')
        .map(|(position, _value)| {
            let count = DIRECTIONS
                .iter()
                .map(|mas_positions| {
                    mas_positions
                        .iter()
                        .map(|offset| positions.get(&(position + offset)))
                        .enumerate()
                        .all(|(index, value)| mas.get(index) == value)
                })
                .filter(|b| *b)
                .count();
            // info!(?position, ?value, count);
            count
        })
        .sum();

    Ok(result.to_string())
}
pub fn get_char(char_matrix: Vec<char>, x: i32, y: i32) -> char {
    '*'
}

pub fn process(input: &str) -> miette::Result<String> {
    let matrix: Vec<&str> = input.lines().collect();
    // let chars: <Vec<Vec<char>> = matrix.iter().map(|s| s.chars()).collect::<Vec<Vec<char>>>();
    let mut chars: Vec<char> = [].to_vec();
    for line in &matrix {
        for char in line.chars() {
            chars.push(char)
        }
    }
    let result = "18";
    let w = matrix[0].len();
    let h = matrix.len();
    info!(" w {} h {}", w, h);

    info!(?matrix, ?chars, " {} {}", matrix.len(), chars.len());

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        assert_eq!("18", process(input)?);
        Ok(())
    }

    #[test_log::test]

    fn test_indexing() -> miette::Result<()> {
        const A: [u32; 5] = [0, 1, 2, 3, 4];
        let mut index: usize = 0;
        index = index + 1;
        let value = A[index];
        info!("A[index]{}", A[index]);

        assert_eq!(1, value);
        Ok(())
    }
}
