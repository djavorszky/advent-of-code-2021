use std::fs;

fn main() {
    let data: Vec<i32> = fs::read_to_string("input.txt")
        .expect("Something went wrong")
        .lines()
        .map(|l| l.parse().unwrap())
        .collect();

    println!("Task 1: {}", task_1(&data));
}

pub fn task_1(data: &Vec<i32>) -> usize {
    data.windows(2).filter(|v| v[1] > v[0]).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let data = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

        assert_eq!(task_1(&data), 7);
    }
}
