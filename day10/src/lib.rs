pub fn task_1(input: &[&str]) -> usize {
    0
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

        assert_eq!(task_1(&data), 26397);
    }

    //    #[test]
    fn task_2_passes() {
        let data = vec![];

        assert_eq!(task_2(&data), 1);
    }
}
