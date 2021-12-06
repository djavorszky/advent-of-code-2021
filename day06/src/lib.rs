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
    let day8 = pop[&8];
    let day7 = pop[&7];
    let day6 = pop[&6];
    let day5 = pop[&5];
    let day4 = pop[&4];
    let day3 = pop[&3];
    let day2 = pop[&2];
    let day1 = pop[&1];
    let day0 = pop[&0];

    pop.insert(8, day0);
    pop.insert(7, day8);
    pop.insert(6, day7 + day0);
    pop.insert(5, day6);
    pop.insert(4, day5);
    pop.insert(3, day4);
    pop.insert(2, day3);
    pop.insert(1, day2);
    pop.insert(0, day1);
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
