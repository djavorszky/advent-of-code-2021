use day11::*;

fn main() {
    let data = include_str!("../input.txt")
        .lines()
        .collect::<Vec<&str>>()
        .join("");

    println!("Task 1: {}", task_1(&data));
    println!("Task 2: {}", task_2(&data));
}
