use std::cmp::Ordering;
use std::str::FromStr;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Line {
    pub start: Point,
    pub end: Point,
    pub line_type: LineType,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum LineType {
    Horizontal,
    Vertical,
    Diagonal,
}

impl LineType {
    fn from(p1: &Point, p2: &Point) -> Self {
        if (p1.x == p2.x) {
            Self::Vertical
        } else if (p1.y == p2.y) {
            Self::Horizontal
        } else {
            Self::Diagonal
        }
    }
}

impl Line {
    fn new(input: Vec<Point>) -> Self {
        let p1 = input[0];
        let p2 = input[1];
        Self {
            start: p1,
            end: p2,
            line_type: LineType::from(&p1, &p2),
        }
    }
}

impl FromStr for Line {
    type Err = ();
    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut parts: Vec<Point> = line.split(" -> ").map(|p| p.parse().unwrap()).collect();

        parts.sort();

        Ok(Line::new(parts))
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.x == other.x && self.y == other.y {
            Ordering::Equal
        } else if self.x > other.x || self.y > other.y {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl FromStr for Point {
    type Err = ();
    fn from_str(points: &str) -> Result<Self, Self::Err> {
        let (x, y) = points.split_once(',').unwrap();

        Ok(Self::new(x.parse().unwrap(), y.parse().unwrap()))
    }
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn parse_line_horizontal_passes() {
        vec![
            (
                "0,9 -> 2,9",
                Line {
                    start: Point::new(0, 9),
                    end: Point::new(2, 9),
                    line_type: LineType::Horizontal,
                },
            ),
            (
                "2,9 -> 0,9",
                Line {
                    start: Point::new(0, 9),
                    end: Point::new(2, 9),
                    line_type: LineType::Horizontal,
                },
            ),
        ]
        .into_iter()
        .for_each(|(input, expected)| {
            assert_eq!(input.parse::<Line>().unwrap(), expected, "{}", input);
        });
    }

    #[test]
    fn parse_line_vertical_passes() {
        vec![
            (
                "2,5 -> 2,9",
                Line {
                    start: Point::new(2, 5),
                    end: Point::new(2, 9),
                    line_type: LineType::Vertical,
                },
            ),
            (
                "2,9 -> 2,5",
                Line {
                    start: Point::new(2, 5),
                    end: Point::new(2, 9),
                    line_type: LineType::Vertical,
                },
            ),
        ]
        .into_iter()
        .for_each(|(input, expected)| {
            assert_eq!(input.parse::<Line>().unwrap(), expected, "{}", input);
        });
    }

    #[test]
    fn parse_line_diagonal_passes() {
        vec![
            (
                "0,0 -> 8,8",
                Line {
                    start: Point::new(0, 0),
                    end: Point::new(8, 8),
                    line_type: LineType::Diagonal,
                },
            ),
            (
                "8,8 -> 0,0",
                Line {
                    start: Point::new(0, 0),
                    end: Point::new(8, 8),
                    line_type: LineType::Diagonal,
                },
            ),
        ]
        .into_iter()
        .for_each(|(input, expected)| {
            assert_eq!(input.parse::<Line>().unwrap(), expected, "{}", input);
        });
    }
}
