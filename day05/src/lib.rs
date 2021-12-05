#![allow(unused)]

mod structs;

use structs::{Line, LineType, Point};

pub fn task_1(input: &[&str], size: usize) -> usize {
    let lines: Vec<Line> = input
        .into_iter()
        .map(|line| line.parse().unwrap())
        .collect();

    let mut grid = vec![0; size * size];

    lines
        .into_iter()
        .filter(|l| l.line_type != LineType::Diagonal)
        .for_each(|line| {
            match line.line_type {
                LineType::Diagonal => (),
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
            }

            for x in line.start.x..line.end.x {
                for y in line.start.y..line.end.y {
                    grid[x + y * size] += 1
                }
            }
        });

    grid.into_iter().filter(|v| *v > 1).count()
}

pub fn task_2(input: &[&str]) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task_1_passes() {
        let data: Vec<&str> = include_str!("test-input.txt").lines().collect();

        assert_eq!(task_1(&data, 10), 5);
    }

    // #[test]
    // fn task_2_passes() {
    //     let data = vec![];

    //     assert_eq!(task_2(&data), 1);
    // }
}
