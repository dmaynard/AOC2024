use std::collections::HashSet;

use glam::IVec2;

const NEIGHBORS: [IVec2; 8] = [
    IVec2::X,
    IVec2::Y,
    IVec2::NEG_X,
    IVec2::NEG_Y,
    IVec2::ONE,
    IVec2::NEG_ONE,
    IVec2::new(1, -1),
    IVec2::new(-1, 1),
];

#[tracing::instrument]
pub fn process_a(input: &str) -> miette::Result<String> {
    let positions = parse(input);

    let count = positions
        .iter()
        .filter(|&position| {
            NEIGHBORS
                .iter()
                .filter(|&offset| {
                    positions.contains(&(position + offset))
                })
                .count()
                < 4
        })
        .count();
    Ok(count.to_string())
}
pub fn process_b(input: &str) -> miette::Result<String> {
    let (floor, positions) = parse2(input);

    let count = positions
        .iter()
        .filter(|&position| {
            NEIGHBORS
                .iter()
                .filter(|&offset| {
                    let pos = position + offset;
                    if pos.x >= 0 
                        && pos.x < floor[0].len() as i32 
                        && pos.y >= 0 
                        && pos.y < floor.len() as i32 {
                        floor[pos.y as usize][pos.x as usize] == 1
                    } else {
                        false
                    }
                })
                .count()
                < 4
        })
        .count();
    Ok(count.to_string())
}
pub fn parse(input: &str) -> HashSet<IVec2> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(
                move |(x, value)| {
                    (value == '@').then_some(IVec2::new(
                        x as i32, y as i32,
                    ))
                },
            )
        })
        .collect::<HashSet<IVec2>>()
}

pub fn parse2(input: &str) -> (Vec<Vec<u8>>, Vec<IVec2>) {
    let mut positions = Vec::new();
    let grid = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| {
                    let value = if c == '@' {
                        positions.push(IVec2::new(x as i32, y as i32));
                        1
                    } else {
                        0
                    };
                    value
                })
                .collect()
        })
        .collect();
    (grid, positions)
}

pub fn parse3(input: &str) -> (Vec<Vec<u8>>, Vec<IVec2>) {
    let mut positions = Vec::new();
    let mut grid: Vec<Vec<u8>> = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            let mut row = vec![0u8]; // Left border
            row.extend(line.chars().enumerate().map(|(x, c)| {
                let value = if c == '@' {
                    // Adjust coordinates: +1 for border
                    positions.push(IVec2::new((x + 1) as i32, (y + 1) as i32));
                    1
                } else {
                    0
                };
                value
            }));
            row.push(0u8); // Right border
            row
        })
        .collect();
    
    // Add top and bottom borders
    if !grid.is_empty() {
        let width = grid[0].len();
        let top_bottom_border = vec![0u8; width];
        grid.insert(0, top_bottom_border.clone());
        grid.push(top_bottom_border);
    }
    
    (grid, positions)
}

pub fn process_c(input: &str) -> miette::Result<String> {
    let (floor, positions) = parse3(input);

    let count = positions
        .iter()
        .filter(|&position| {
            NEIGHBORS
                .iter()
                .filter(|&offset| {
                    let pos = *position + *offset;
                    // No bounds checking needed - sentinel border guarantees valid access
                    floor[pos.y as usize][pos.x as usize] == 1
                })
                .count()
                < 4
        })
        .count();
    Ok(count.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let input = "..@@.@@@@.
    @@@.@.@.@@
    @@@@@.@.@@
    @.@@@@..@.
    @@.@@@@.@@
    .@@@@@@@.@
    .@.@.@.@@@
    @.@@@.@@@@
    .@@@@@@@@.
    @.@.@@@.@.";
    // eprintln!("{:?}", parse2(input));
    // dbg!(parse2(input));
    assert_eq!("13", process_a(input)?);
        Ok(())
    }
    #[test_log::test]
    fn test_process_a() -> miette::Result<()> {
        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
        assert_eq!("13", process_a(input)?);
        Ok(())
    }
   
}

