pub fn task_1(input: &mut [i32]) -> usize {
    input.sort_unstable();

    let mean = input[input.len() / 2];

    input.iter().fold(0, |acc, v| acc + (v - mean).abs()) as usize
}

pub fn task_2(_input: &[i32]) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task_1_passes() {
        let mut data = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];

        assert_eq!(task_1(&mut data), 37);
    }

    #[test]
    fn task_2_passes() {
        let data = vec![];

        assert_eq!(task_2(&data), 2);
    }
}
