use std::collections::{HashMap, HashSet};

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

    explore_map(&connections)
}

fn explore_map(c: &HashMap<&str, Vec<&str>>) -> usize {
    fn visit_cave<'a>(
        current: &'a str,
        seen: &mut HashSet<&'a str>,
        c: &HashMap<&'a str, Vec<&'a str>>,
    ) -> usize {
        if current == "end" {
            return 1;
        }

        if seen.contains(current) {
            return 0;
        }

        if current.to_ascii_lowercase() == current {
            seen.insert(current);
        }

        let result = c[current]
            .iter()
            .filter(|cave| **cave != "start")
            .fold(0, |acc, next| acc + visit_cave(next, seen, c));

        seen.remove(current);

        result
    }

    visit_cave("start", &mut HashSet::new(), &c)
}

pub fn task_2(_input: &[&str]) -> usize {
    0
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
