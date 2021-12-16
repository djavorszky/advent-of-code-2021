mod map;

use map::{Map, ALL_DIRECTIONS};

pub fn task_1(input: &[&str]) -> u32 {
    let map: Map = input.into();

    map.levels
        .iter()
        .enumerate()
        .filter(|(idx, level)| {
            let level = **level;
            for d in ALL_DIRECTIONS {
                if let Some(value) = map.neighbour_val(*idx, d) {
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
    let map: Map = input.into();

    let mut basin_sizes: Vec<usize> = map
        .levels
        .iter()
        .enumerate()
        .filter(|(idx, level)| {
            let level = **level;
            for d in ALL_DIRECTIONS {
                if let Some(value) = map.neighbour_val(*idx, d) {
                    if value <= level {
                        return false;
                    }
                }
            }

            true
        })
        .map(|(idx, _)| map.get_basin_size(idx))
        .collect();

    basin_sizes.sort_unstable();

    basin_sizes[basin_sizes.len() - 3..].iter().product()
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

    #[test]
    fn task_2_passes() {
        let input = include_str!("test_input.txt");
        let data: Vec<&str> = input.lines().collect();

        assert_eq!(task_2(&data), 1134);
    }
}
