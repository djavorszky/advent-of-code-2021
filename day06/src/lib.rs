#![allow(unused)]

pub fn task_1(input: &[u8]) -> usize {
    let mut data: Vec<Fish> = input.iter().map(|n| Fish(*n)).collect();

    for _ in 0..80 {
        simulate(&mut data);
    }

    data.len()
}

fn simulate(population: &mut Vec<Fish>) {
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

pub fn task_2(input: &[u8]) -> usize {
    0
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
        let data = vec![];

        assert_eq!(task_2(&data), 2);
    }
}
