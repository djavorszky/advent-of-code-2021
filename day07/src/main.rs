use day07::*;

fn main() {
    let input = include_str!("../input.txt");
    let mut data: Vec<i32> = input
        .split(',')
        .map(|v| v.parse::<i32>().unwrap())
        .collect();

    println!("Task 1: {}", task_1(&mut data));
    println!("Task 2: {}", task_2(&data));
}
