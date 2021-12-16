#![allow(unused)]

mod structs;

use structs::{Line, LineType, Point};

pub fn task_1(input: &[&str], size: usize) -> usize {
    let mut grid = vec![0; size * size];

    let lines = input
        .iter()
        .map(|line| line.parse::<Line>().unwrap())
        .filter(|l| matches!(l.line_type, LineType::Horizontal | LineType::Vertical))
        .for_each(|line| {
            match line.line_type {
                LineType::Horizontal => {
                    for x in line.start.x..=line.end.x {
                        grid[x + line.start.y * size] += 1
                    }
                }
                LineType::Vertical => {
                    for y in line.start.y..=line.end.y {
                        grid[line.start.x + y * size] += 1
                    }
                }
                _ => (),
            }

            for x in line.start.x..line.end.x {
                for y in line.start.y..line.end.y {
                    grid[x + y * size] += 1
                }
            }
        });

    grid.into_iter().filter(|v| *v > 1).count()
}

pub fn task_2(input: &[&str], size: usize) -> usize {
    let mut grid = vec![0; size * size];

    let lines = input
        .iter()
        .map(|line| line.parse::<Line>().unwrap())
        .for_each(|line| match line.line_type {
            LineType::Horizontal => {
                for x in line.start.x..=line.end.x {
                    grid[x + line.start.y * size] += 1
                }
            }
            LineType::Vertical => {
                for y in line.start.y..=line.end.y {
                    grid[line.start.x + y * size] += 1
                }
            }
            LineType::DiagonalAsc => {
                for (idx, x) in (line.start.x..=line.end.x).enumerate() {
                    grid[x + (line.start.y - idx) * size] += 1;
                }
            }
            LineType::DiagonalDesc => {
                for (idx, x) in (line.start.x..=line.end.x).enumerate() {
                    grid[x + (line.start.y + idx) * size] += 1;
                }
            }
        });

    grid.into_iter().filter(|v| *v > 1).count()
}

fn range(x: usize, y: usize) -> std::ops::RangeInclusive<usize> {
    if (x > y) {
        y..=x
    } else {
        x..=y
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task_1_passes() {
        let data: Vec<&str> = include_str!("test-input.txt").lines().collect();

        assert_eq!(task_1(&data, 10), 5);
    }

    #[test]
    fn task_2_passes() {
        let data: Vec<&str> = include_str!("test-input.txt").lines().collect();

        assert_eq!(task_2(&data, 10), 12);
    }
}
