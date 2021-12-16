pub fn task_1(input: &mut [i32]) -> usize {
    input.sort_unstable();

    let mean = input[input.len() / 2];

    input.iter().fold(0, |acc, v| acc + (v - mean).abs()) as usize
}

pub fn task_2(input: &mut [i32]) -> i32 {
    input.sort_unstable();
    let max = *input.iter().max().unwrap() as usize;
    let mut min = i32::MAX;
    for location in 0..=max {
        let dist = get_distance_to(location as i32, input);
        if dist < min {
            min = dist;
        }
    }

    min
}

fn get_distance_to(location: i32, input: &[i32]) -> i32 {
    input.iter().fold(0, |acc, crab_loc| {
        let distance = (*crab_loc - location).abs();

        acc + (distance * (distance + 1)) / 2
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task_1_passes() {
        let mut data = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];

        assert_eq!(task_1(&mut data), 37);
    }

    #[test]
    fn task_2_passes() {
        let mut data = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];

        assert_eq!(task_2(&mut data), 168);
    }
}
