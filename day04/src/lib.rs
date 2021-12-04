#![allow(unused)]

pub fn task_1(input: &[&str]) -> usize {
    let instructions: Vec<usize> = input[0].split(',').map(|n| n.parse().unwrap()).collect();

    let mut boards: Vec<BingoBoard> = input[2..].chunks(6).map(|w| w.into()).collect();

    run_task_1(instructions, boards).unwrap()
}

fn run_task_1(instructions: Vec<usize>, mut boards: Vec<BingoBoard>) -> Option<usize> {
    for i in instructions {
        for mut b in &mut boards {
            mark_field(b, i);

            if is_bingo(b) {
                let unmarked_sum: usize = b
                    .all_fields
                    .iter()
                    .filter(|v| !b.marked_fields.contains(v))
                    .sum();

                return Some(unmarked_sum * i);
            }
        }
    }

    None
}

pub fn task_2(input: &[&str]) -> usize {
    let instructions: Vec<usize> = input[0].split(',').map(|n| n.parse().unwrap()).collect();

    let mut boards: Vec<BingoBoard> = input[2..].chunks(6).map(|w| w.into()).collect();

    run_task_2(instructions, boards).unwrap()
}

fn run_task_2(instructions: Vec<usize>, mut boards: Vec<BingoBoard>) -> Option<usize> {
    let mut boards_won = 0;
    let boards_size = boards.len();

    for i in instructions {
        for mut b in &mut boards {
            if b.won_already {
                continue;
            }

            mark_field(b, i);

            if is_bingo(b) {
                b.won_already = true;
                boards_won += 1;

                if boards_won == boards_size {
                    let unmarked_sum: usize = b
                        .all_fields
                        .iter()
                        .filter(|v| !b.marked_fields.contains(v))
                        .sum();

                    return Some(unmarked_sum * i);
                }
            }
        }
    }

    None
}

#[derive(Debug, Clone)]
struct BingoBoard {
    all_fields: Vec<usize>,
    marked_fields: Vec<usize>,
    won_already: bool,
}

impl From<&[&str]> for BingoBoard {
    fn from(input: &[&str]) -> Self {
        Self {
            marked_fields: vec![],
            all_fields: input.iter().fold(
                Vec::with_capacity(25),
                |mut acc: Vec<usize>, line: &&str| {
                    line.split_whitespace()
                        .map(|i| i.parse::<usize>().unwrap())
                        .for_each(|n| acc.push(n));

                    acc
                },
            ),
            won_already: false,
        }
    }
}

fn mark_field(board: &mut BingoBoard, n: usize) {
    board.marked_fields.push(n);
}

fn is_bingo(board: &BingoBoard) -> bool {
    if board.marked_fields.len() < 5 {
        return false;
    }

    has_marked_col(board) || has_marked_row(board) || has_marked_diagonal(board)
}

fn has_marked_row(board: &BingoBoard) -> bool {
    board
        .all_fields
        .chunks(5)
        .any(|rows| rows.iter().all(|v| board.marked_fields.contains(v)))
}

fn has_marked_col(board: &BingoBoard) -> bool {
    (0..5).any(|start| {
        board
            .all_fields
            .iter()
            .skip(start)
            .step_by(5)
            .all(|v| board.marked_fields.contains(v))
    })
}

fn has_marked_diagonal(board: &BingoBoard) -> bool {
    board
        .all_fields
        .iter()
        .skip(5)
        .step_by(4)
        .all(|v| board.marked_fields.contains(v))
        || (0..5).all(|v| board.marked_fields.contains(&board.all_fields[v * 5 + v]))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task_1_passes() {
        let data: Vec<&str> = include_str!("test-input.txt").lines().collect();

        assert_eq!(task_1(&data), 4512);
    }

    #[test]
    fn task_2_passes() {
        let data: Vec<&str> = include_str!("test-input.txt").lines().collect();

        assert_eq!(task_2(&data), 1924);
    }
}
