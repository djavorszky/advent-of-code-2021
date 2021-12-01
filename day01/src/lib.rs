pub fn task_1(data: &[i32]) -> usize {
    data.windows(2).filter(|v| v[1] > v[0]).count()
}

pub fn task_2(data: &[i32]) -> usize {
    data.windows(4)
        .filter(|v| v[1] + v[2] + v[3] > v[0] + v[1] + v[2])
        .count()
}

pub fn task_2_alternative(data: &[i32]) -> usize {
    data.windows(3)
        .map(|v| v.iter().sum::<i32>())
        .collect::<Vec<i32>>()
        .windows(2)
        .filter(|v| v[1] > v[0])
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task_1_passes() {
        let data = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

        assert_eq!(task_1(&data), 7);
    }

    #[test]
    fn task_2_passes() {
        let data = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

        assert_eq!(task_2(&data), 5);
    }

    #[test]
    fn task_2_alternative_passes() {
        let data = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

        assert_eq!(task_2_alternative(&data), 5);
    }
}
