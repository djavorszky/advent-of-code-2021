use day10::*;

fn main() {
    let input = include_str!("../input.txt");
    let data: Vec<&str> = input.lines().collect();

    println!("Task 1: {}", task_1(&data));
    println!("Task 2: {}", task_2(&data));
}
