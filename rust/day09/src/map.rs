use std::collections::HashSet;

pub struct Map {
    pub levels: Vec<u32>,
    width: usize,
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub const ALL_DIRECTIONS: [Direction; 4] = [
    Direction::North,
    Direction::East,
    Direction::South,
    Direction::West,
];

impl Map {
    pub fn neighbour_val(&self, index: usize, d: Direction) -> Option<u32> {
        self.neighbour(index, d).map(|v| v.1)
    }

    fn neighbour(&self, index: usize, d: Direction) -> Option<(usize, u32)> {
        if !self.has_neighbour(index, d) {
            return None;
        }
        match d {
            Direction::North => Some((index - self.width, self.levels[index - self.width])),
            Direction::East => Some((index + 1, self.levels[index + 1])),
            Direction::South => Some((index + self.width, self.levels[index + self.width])),
            Direction::West => Some((index - 1, self.levels[index - 1])),
        }
    }

    fn has_neighbour(&self, index: usize, d: Direction) -> bool {
        match d {
            Direction::North => index >= self.width,
            Direction::East => index % self.width != self.width - 1,
            Direction::South => index < self.levels.len() - self.width,
            Direction::West => index % self.width != 0,
        }
    }

    pub fn get_basin_size(&self, start_idx: usize) -> usize {
        let mut seen = HashSet::with_capacity(100);

        self.walk_basin(start_idx, &mut seen)
    }

    fn walk_basin(&self, current_idx: usize, seen: &mut HashSet<usize>) -> usize {
        if seen.contains(&current_idx) {
            return 0;
        }

        seen.insert(current_idx);

        ALL_DIRECTIONS.iter().fold(1, |acc, direction| {
            let result = match self.neighbour(current_idx, *direction) {
                None => 0,
                Some((idx, value)) if value == 9 || seen.contains(&idx) => 0,
                Some((idx, _)) => self.walk_basin(idx, seen),
            };

            acc + result
        })
    }
}

impl From<&[&str]> for Map {
    fn from(data: &[&str]) -> Self {
        let width = data.iter().next().unwrap().len();

        Self {
            width,
            levels: data
                .iter()
                .flat_map(|c| {
                    c.chars()
                        .map(|c| c.to_digit(10).unwrap())
                        .collect::<Vec<u32>>()
                })
                .collect(),
        }
    }
}

impl From<Vec<&str>> for Map {
    fn from(data: Vec<&str>) -> Self {
        Map::from(&data as &[&str])
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_has_neighbour() {
        let input = include_str!("test_input.txt");
        let map: Map = input.lines().collect::<Vec<&str>>().into();

        [
            (0, false, true, true, false),
            (9, false, false, true, true),
            (10, true, true, true, false),
            (15, true, true, true, true),
            (40, true, true, false, false),
            (49, true, false, false, true),
        ]
        .into_iter()
        .for_each(|(idx, north, east, south, west)| {
            assert_eq!(
                map.has_neighbour(idx, Direction::North),
                north,
                "idx: {}, dir: north",
                idx
            );
            assert_eq!(
                map.has_neighbour(idx, Direction::East),
                east,
                "idx: {}, dir: east",
                idx
            );
            assert_eq!(
                map.has_neighbour(idx, Direction::South),
                south,
                "idx: {}, dir: south",
                idx
            );
            assert_eq!(
                map.has_neighbour(idx, Direction::West),
                west,
                "idx: {}, dir: west",
                idx
            );
        });
    }

    #[test]
    fn test_get_neighbour() {
        let input = include_str!("test_input.txt");
        let map: Map = input.lines().collect::<Vec<&str>>().into();

        [
            (0, None, Some(1), Some(3), None),
            (9, None, None, Some(1), Some(1)),
            (10, Some(2), Some(9), Some(9), None),
            (40, Some(8), Some(8), None, None),
            (49, Some(9), None, None, Some(7)),
        ]
        .into_iter()
        .for_each(|(idx, north, east, south, west)| {
            assert_eq!(
                map.neighbour_val(idx, Direction::North),
                north,
                "idx: {}, dir: north",
                idx
            );
            assert_eq!(
                map.neighbour_val(idx, Direction::East),
                east,
                "idx: {}, dir: east",
                idx
            );
            assert_eq!(
                map.neighbour_val(idx, Direction::South),
                south,
                "idx: {}, dir: south",
                idx
            );
            assert_eq!(
                map.neighbour_val(idx, Direction::West),
                west,
                "idx: {}, dir: west",
                idx
            );
        });
    }

    #[test]
    fn test_get_basin_size() {
        let input = include_str!("test_input.txt");
        let map: Map = input.lines().collect::<Vec<&str>>().into();

        [(0, 3), (9, 9), (22, 14), (46, 9)]
            .into_iter()
            .for_each(|(start_idx, expected)| {
                assert_eq!(
                    map.get_basin_size(start_idx),
                    expected,
                    "idx: {}",
                    start_idx
                );
            })
    }
}
