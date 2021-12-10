pub fn task_1(input: &[&str]) -> usize {
    input
        .iter()
        .filter_map(|line| get_corrupt_score(line))
        .sum()
}

fn get_corrupt_score(line: &str) -> Option<usize> {
    let mut stack = Vec::with_capacity(10);

    for c in line.chars() {
        if is_open(c) {
            stack.push(c);
        } else {
            let last_char = stack.pop()?;

            let expected_closing = closing_char(last_char);
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
    matches!(c, '(' | '[' | '{' | '<')
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
    let mut scores: Vec<usize> = input
        .iter()
        .filter_map(|line| get_autocomplete_score(line))
        .collect();

    scores.sort_unstable();

    scores[scores.len() / 2]
}

fn get_autocomplete_score(line: &str) -> Option<usize> {
    let mut stack = Vec::with_capacity(10);

    for c in line.chars() {
        if is_open(c) {
            stack.push(c);
        } else {
            let last_char = stack.pop()?;

            let expected_closing = closing_char(last_char);
            if c != expected_closing {
                return None;
            }
        }
    }

    if !stack.is_empty() {
        return Some(autocomplete_score(stack));
    }

    None
}

fn autocomplete_score(stack: Vec<char>) -> usize {
    stack
        .iter()
        .rev()
        .map(|c| closing_char(*c))
        .fold(0, |acc, c| acc * 5 + autocomplete_value(c))
}

fn autocomplete_value(c: char) -> usize {
    match c {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => panic!("oh no"),
    }
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

    #[test]
    fn task_2_passes() {
        let input = include_str!("test_input.txt");
        let data: Vec<&str> = input.lines().collect();

        assert_eq!(task_2(&data), 288957);
    }
}
