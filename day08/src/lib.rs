#![allow(unused)]

pub fn task_1(input: &[&str]) -> usize {
    let known_lengths: [usize; 4] = [2, 3, 4, 7];

    input
        .iter()
        .map(|line| line.split('|').nth(1).unwrap())
        .fold(0, |acc, line| {
            acc + line
                .split(' ')
                .filter(|segment| known_lengths.contains(&segment.len()))
                .count()
        })
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

        assert_eq!(task_1(&data), 26);
    }

    // #[test]
    // fn task_2_passes() {
    //     let input = include_str!("test_input.txt");
    //     let data: Vec<&str> = input.lines().collect();

    //     assert_eq!(task_2(&data), 1);
    // }
}
