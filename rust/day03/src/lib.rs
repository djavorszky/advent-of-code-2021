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

use std::cmp::Ordering;

pub fn task_2(input: &[&str]) -> usize {
    let parsed_inputs: Vec<usize> = input
        .iter()
        .map(|i| usize::from_str_radix(i, 2).unwrap())
        .collect();

    let len = input[0].len();

    let o2 = find_result(parsed_inputs.clone(), System::Generator, len - 1);
    let co2 = find_result(parsed_inputs, System::Scrubber, len - 1);

    o2 * co2
}

fn find_result(input: Vec<usize>, system: System, bit_position: usize) -> usize {
    if input.is_empty() {
        panic!("empty input");
    }
    let mask = get_mask(&input, bit_position);

    let res: Vec<usize> = input
        .into_iter()
        .filter(|num| system.should_include(mask, get_bit_value(*num, bit_position)))
        .collect();

    if res.len() == 1 {
        return res[0];
    }

    find_result(res, system, bit_position - 1)
}

fn get_mask(input: &[usize], bit_location: usize) -> Mask {
    let sum = input.iter().fold(0, |acc, i| {
        if is_bit_one(*i, bit_location) {
            acc + 1
        } else {
            acc
        }
    });

    let half_size = (input.len() + 1) / 2;

    match sum.cmp(&half_size) {
        Ordering::Greater => Mask::Ones,
        Ordering::Less => Mask::Zeros,
        Ordering::Equal => Mask::Equal,
    }
}

#[inline]
fn is_bit_one(input: usize, bit_loc: usize) -> bool {
    get_bit_value(input, bit_loc) == 1
}

#[inline]
fn get_bit_value(input: usize, bit_loc: usize) -> usize {
    (input & (1 << bit_loc)) >> bit_loc
}

#[derive(Copy, Clone, Debug)]
enum Mask {
    Ones,
    Zeros,
    Equal,
}

#[derive(Copy, Clone)]
enum System {
    Scrubber,
    Generator,
}

impl System {
    fn should_include(&self, mask: Mask, bit: usize) -> bool {
        match self {
            System::Scrubber => match mask {
                Mask::Ones | Mask::Equal => bit == 0,
                Mask::Zeros => bit == 1,
            },
            System::Generator => match mask {
                Mask::Ones | Mask::Equal => bit == 1,
                Mask::Zeros => bit == 0,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bit_one_passes() {
        vec![
            (0b1, 0, true),
            (0b10, 1, true),
            (0b10, 0, false),
            (0b11, 0, true),
            (0b11, 1, true),
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
    fn get_bit_value_passes() {
        let test_data = 0b100101;
        vec![(0, 1), (1, 0), (2, 1), (3, 0), (4, 0), (5, 1)]
            .into_iter()
            .for_each(|(bit_loc, expected)| {
                assert_eq!(
                    get_bit_value(test_data, bit_loc),
                    expected,
                    "input: {:b}, loc: {}",
                    test_data,
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
        let data = vec![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ];

        assert_eq!(task_2(&data), 230);
    }
}
