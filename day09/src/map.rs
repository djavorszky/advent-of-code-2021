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

impl Map {
    pub fn neighbour(&self, index: usize, d: Direction) -> Option<u32> {
        if !self.has_neighbour(index, d) {
            return None;
        }
        match d {
            Direction::North => Some(self.levels[index - self.width]),
            Direction::East => Some(self.levels[index + 1]),
            Direction::South => Some(self.levels[index + self.width]),
            Direction::West => Some(self.levels[index - 1]),
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
                map.neighbour(idx, Direction::North),
                north,
                "idx: {}, dir: north",
                idx
            );
            assert_eq!(
                map.neighbour(idx, Direction::East),
                east,
                "idx: {}, dir: east",
                idx
            );
            assert_eq!(
                map.neighbour(idx, Direction::South),
                south,
                "idx: {}, dir: south",
                idx
            );
            assert_eq!(
                map.neighbour(idx, Direction::West),
                west,
                "idx: {}, dir: west",
                idx
            );
        });
    }
}
