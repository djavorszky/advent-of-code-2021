use std::{fs, i32};

use day01::*;

fn main() {
    let data: Vec<i32> = fs::read_to_string("input.txt")
        .expect("Something went wrong")
        .lines()
        .map(|l| l.parse().unwrap())
        .collect();

    println!("Task 1: {}", task_1(&data));
    println!("Task 2: {}", task_2(&data));
    println!("Task 2 Alt: {}", task_2_alternative(&data));
}
