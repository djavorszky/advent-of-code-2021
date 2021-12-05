use std::fs;

use day05::*;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let data: Vec<&str> = input.lines().collect();

    println!("Task 1: {}", task_1(&data, 991));
    println!("Task 2: {}", task_2(&data));
}
