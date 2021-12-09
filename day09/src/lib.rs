mod map;

use map::{Direction, Map};

pub fn task_1(input: &[&str]) -> u32 {
    let map: Map = input.into();

    let directions = [
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West,
    ];

    map.levels
        .iter()
        .enumerate()
        .filter(|(idx, level)| {
            let level = **level;
            for d in directions {
                if let Some(value) = map.neighbour(*idx, d) {
                    if value <= level {
                        return false;
                    }
                }
            }

            true
        })
        .map(|(_, level)| level + 1)
        .sum::<u32>()
}

pub fn task_2(input: &[&str]) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task_1_passes() {
        let input = include_str!("test_input.txt");
        let data: Vec<&str> = input.lines().collect();

        assert_eq!(task_1(&data), 15);
    }

    // #[test]
    // fn task_2_passes() {
    //     let input = include_str!("test_input.txt");
    //     let data: Vec<&str> = input.lines().collect();

    //     assert_eq!(task_2(&data), 1);
    // }
}
