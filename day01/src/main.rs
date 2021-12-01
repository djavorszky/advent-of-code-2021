use std::fs;

fn main() {
    let data: Vec<i32> = fs::read_to_string("input.txt")
        .expect("Something went wrong")
        .lines()
        .map(|l| l.parse().unwrap())
        .collect();

    println!("Task 1: {}", task_1(&data));
    println!("Task 2: {}", task_2(&data));
}

fn task_1(data: &[i32]) -> usize {
    data.windows(2).filter(|v| v[1] > v[0]).count()
}

fn task_2(data: &[i32]) -> usize {
    data.windows(4)
        .filter(|v| v[1] + v[2] + v[3] > v[0] + v[1] + v[2])
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
}
