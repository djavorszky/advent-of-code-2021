pub fn task_1(input: &[&str]) -> usize {
    input
        .into_iter()
        .filter_map(|line| get_corrupt_score(line))
        .sum()
}

fn get_corrupt_score(line: &str) -> Option<usize> {
    let mut stack = Vec::with_capacity(10);

    for c in line.chars() {
        if is_open(c) {
            stack.push(c);
        } else {
            let last_char = stack.pop();
            if last_char.is_none() {
                return None;
            }

            let expected_closing = closing_char(last_char.unwrap());
            if c != expected_closing {
                return Some(corrupt_score(c));
            }
        }
    }

    None
}

fn closing_char(open: char) -> char {
    match open {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => panic!("oh no, char: {}", open),
    }
}

fn is_open(c: char) -> bool {
    match c {
        '(' | '[' | '{' | '<' => true,
        _ => false,
    }
}

fn corrupt_score(c: char) -> usize {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("oh no"),
    }
}

pub fn task_2(input: &[&str]) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task_1_passes() {
        let input = include_str!("test_input.txt");
        let data: Vec<&str> = input.lines().collect();

        assert_eq!(task_1(&data), 26397);
    }

    //    #[test]
    fn task_2_passes() {
        let data = vec![];

        assert_eq!(task_2(&data), 1);
    }
}
