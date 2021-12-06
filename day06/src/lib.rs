#![allow(unused)]

use std::collections::HashMap;

pub fn task_1(input: &[u8]) -> u128 {
    let mut data = init_map(input);

    for _ in 0..80 {
        simulate_map(&mut data);
    }

    data.values().sum::<u128>()
}

fn init_map(input: &[u8]) -> HashMap<u8, u128> {
    let mut data = HashMap::new();

    for i in 0..9 {
        data.insert(i as u8, 0u128);
    }

    for n in input.iter() {
        data.entry(*n).and_modify(|val| *val += 1);
    }

    data
}

fn simulate_map(pop: &mut HashMap<u8, u128>) {
    let new_fish = pop[&0];
    for i in 1..=8 {
        pop.insert(i - 1, pop[&i]);
    }
    pop.entry(6u8).and_modify(|val| *val += new_fish);
    pop.insert(8, new_fish);
}

fn simulate_vec(population: &mut Vec<Fish>) {
    let mut new_fish = 0;
    for mut fish in population.iter_mut() {
        if fish.0 == 0 {
            new_fish += 1;
            fish.0 = 6;
        } else {
            fish.0 -= 1
        }
    }

    for _ in 0..new_fish {
        population.push(Fish(8));
    }
}

#[derive(Debug)]
struct Fish(u8);

pub fn task_2(input: &[u8]) -> u128 {
    let mut data = init_map(input);

    for _ in 0..256 {
        simulate_map(&mut data);
    }

    data.values().sum::<u128>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task_1_passes() {
        let data = [3, 4, 3, 1, 2];

        assert_eq!(task_1(&data), 5934);
    }

    #[test]
    fn task_2_passes() {
        let data = [3, 4, 3, 1, 2];

        assert_eq!(task_2(&data), 26984457539);
    }
}
