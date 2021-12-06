use day06::*;

fn main() {
    let input = include_str!("../input.txt");
    let data: Vec<usize> = input
        .split(',')
        .map(|n| n.parse::<usize>().unwrap())
        .collect();

    println!("Task 1: {}", task_1(&data));
    println!("Task 2: {}", task_2(&data));
}
