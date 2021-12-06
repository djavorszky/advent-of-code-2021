use day06::*;

fn main() {
    let input = include_str!("../input.txt");
    let data: Vec<u8> = input.split(',').map(|n| n.parse::<u8>().unwrap()).collect();

    println!("Task 1: {}", task_1(&data));
    println!("Task 2: {}", task_2(&data));
}
