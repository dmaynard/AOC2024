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
    let mut positions = parse(input);
    let mut total_removed = 0;

    loop {
        let to_remove: Vec<IVec2> = positions
            .iter()
            .filter(|&position| {
                NEIGHBORS
                    .iter()
                    .filter(|&offset| positions.contains(&(position + offset)))
                    .count()
                    < 4
            })
            .copied()
            .collect();

        if to_remove.is_empty() {
            break;
        }

        total_removed += to_remove.len();
        for pos in to_remove {
            positions.remove(&pos);
        }
    }

    Ok(total_removed.to_string())
}
pub fn process_b(input: &str) -> miette::Result<String> {
    let (mut floor, positions) = parse3(input);
    let mut active_positions: HashSet<IVec2> = positions.iter().copied().collect();
    let mut total_removed = 0;

    loop {
        let to_remove: Vec<IVec2> = active_positions
            .iter()
            .filter(|&position| {
                NEIGHBORS
                    .iter()
                    .filter(|&offset| {
                        let pos = *position + *offset;
                        // Check both that grid cell is 1 AND position is still active
                        floor[pos.y as usize][pos.x as usize] == 1
                            && active_positions.contains(&pos)
                    })
                    .count()
                    < 4
            })
            .copied()
            .collect();

        if to_remove.is_empty() {
            break;
        }

        total_removed += to_remove.len();
        for pos in to_remove {
            // Mark as removed: remove from active set and update grid
            active_positions.remove(&pos);
            floor[pos.y as usize][pos.x as usize] = 0;
        }
    }

    Ok(total_removed.to_string())
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
    let (mut floor, positions) = parse3(input);
    
    // Vec<bool> parallel to positions Vec: active[i] = true if positions[i] still has a roll
    let mut active: Vec<bool> = vec![true; positions.len()];
    let mut total_removed = 0;

    loop {
        let mut any_removed = false;

        // Iterate through all positions and remove them immediately if they have < 4 neighbors
        for (idx, position) in positions.iter().enumerate() {
            // Only check positions that are still active
            if active[idx] {
                let neighbor_count = NEIGHBORS
                    .iter()
                    .filter(|&offset| {
                        let neighbor_pos = *position + *offset;
                        // Just check if there's a roll at that position
                        floor[neighbor_pos.y as usize][neighbor_pos.x as usize] == 1
                    })
                    .count();

                if neighbor_count < 4 {
                    // Remove immediately: set boolean to false and update grid
                    active[idx] = false;
                    floor[position.y as usize][position.x as usize] = 0;
                    total_removed += 1;
                    any_removed = true;
                }
            }
        }

        if !any_removed {
            break;
        }
    }

    Ok(total_removed.to_string())
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
    assert_eq!("43", process_a(input)?);
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
        assert_eq!("43", process_c(input)?);
        Ok(())
    }
   
}

