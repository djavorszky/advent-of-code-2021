use std::collections::HashMap;

pub fn task_1(input: &[&str]) -> usize {
    let mut connections: HashMap<&str, Vec<&str>> = HashMap::with_capacity(10);

    input
        .iter()
        .map(|line| line.split_once('-').unwrap())
        .for_each(|(c1, c2)| {
            connections
                .entry(c1)
                .and_modify(|v| v.push(c2))
                .or_insert_with(|| vec![c2]);

            connections
                .entry(c2)
                .and_modify(|v| v.push(c1))
                .or_insert_with(|| vec![c1]);
        });

    explore_map(&connections, 1)
}

fn explore_map(c: &HashMap<&str, Vec<&str>>, max_size: usize) -> usize {
    fn visit_cave<'a>(
        current: &'a str,
        seen: &mut HashMap<&'a str, usize>,
        c: &HashMap<&'a str, Vec<&'a str>>,
        max_size: usize,
    ) -> usize {
        if current == "end" {
            return 1;
        }

        if seen.get(current).map(|v| *v == max_size).unwrap_or(false) {
            return 0;
        }

        if current.to_ascii_lowercase() == current {
            seen.entry(current).and_modify(|v| *v += 1).or_insert(1);
        }

        let result = c[current]
            .iter()
            .filter(|cave| **cave != "start")
            .fold(0, |acc, next| acc + visit_cave(next, seen, c, max_size));

        seen.remove(current);

        result
    }

    visit_cave("start", &mut HashMap::with_capacity(10), &c, max_size)
}

pub fn task_2(input: &[&str]) -> usize {
    let mut connections: HashMap<&str, Vec<&str>> = HashMap::with_capacity(10);

    input
        .iter()
        .map(|line| line.split_once('-').unwrap())
        .for_each(|(c1, c2)| {
            connections
                .entry(c1)
                .and_modify(|v| v.push(c2))
                .or_insert_with(|| vec![c2]);

            connections
                .entry(c2)
                .and_modify(|v| v.push(c1))
                .or_insert_with(|| vec![c1]);
        });

    explore_map(&connections, 2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task_1_passes() {
        let input = include_str!("test_input.txt");
        let data: Vec<&str> = input.lines().collect();

        assert_eq!(task_1(&data), 19);
    }

    #[test]
    fn task_2_passes() {
        let input = include_str!("test_input.txt");
        let data: Vec<&str> = input.lines().collect();

        assert_eq!(task_2(&data), 103);
    }
}
