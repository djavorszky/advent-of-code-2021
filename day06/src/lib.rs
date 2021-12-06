pub fn task_1(input: &[u8]) -> usize {
    0
}

pub fn task_2(input: &[u8]) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task_1_passes() {
        let data = [3, 4, 3, 1, 2];

        assert_eq!(task_1(&data), 5934);
    }

    #[test]
    fn task_2_passes() {
        let data = vec![];

        assert_eq!(task_2(&data), 2);
    }
}
