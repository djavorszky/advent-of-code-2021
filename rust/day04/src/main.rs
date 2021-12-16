use std::fs;

use day04::*;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let data: Vec<&str> = input.lines().collect();

    println!("Task 1: {}", task_1(&data));
    println!("Task 2: {}", task_2(&data));
}
