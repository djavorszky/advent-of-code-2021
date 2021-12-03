#![allow(unused)]

pub fn task_1(input: &[&str]) -> usize {
    let size = input.len();
    let len = input[0].len();
    let (gamma, epsilon) = input
        .iter()
        .map(|i| usize::from_str_radix(i, 2).unwrap())
        .fold(vec![0; len], |mut acc, c| {
            (0..len)
                .filter(|i| is_bit_one(c, *i))
                .for_each(|i| acc[i] += 1);
            acc
        })
        .into_iter()
        .map(|v| if v > size / 2 { 1 } else { 0 })
        .enumerate()
        .fold((0, 0), |acc, (idx, n)| {
            (acc.0 + (n << idx), acc.1 + ((1 - n) << idx))
        });

    gamma * epsilon
}

#[inline]
fn is_bit_one(input: usize, bit_loc: usize) -> bool {
    input & (1 << bit_loc) != 0
}

pub fn task_2(input: &[&str]) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bit_one_passes() {
        vec![
            (1, 0, true),
            (2, 1, true),
            (2, 0, false),
            (3, 0, true),
            (3, 1, true),
        ]
        .into_iter()
        .for_each(|(input, bit_loc, expected)| {
            assert_eq!(
                is_bit_one(input, bit_loc),
                expected,
                "input: {:b}, loc: {}",
                input,
                bit_loc
            )
        })
    }

    #[test]
    fn task_1_passes() {
        let data = vec![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ];

        assert_eq!(task_1(&data), 198);
    }

    #[test]
    fn task_2_passes() {
        let data = vec![];

        assert_eq!(task_2(&data), 0);
    }
}
