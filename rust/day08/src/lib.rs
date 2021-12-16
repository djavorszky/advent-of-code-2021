pub fn task_1(input: &[&str]) -> usize {
    let known_lengths: [usize; 4] = [2, 3, 4, 7];

    input
        .iter()
        .map(|line| line.split('|').nth(1).unwrap())
        .fold(0, |acc, line| {
            acc + line
                .split(' ')
                .filter(|segment| known_lengths.contains(&segment.len()))
                .count()
        })
}

pub fn task_2(input: &[&str]) -> usize {
    input
        .iter()
        .map(|line| line.split_once(" | ").unwrap())
        .fold(0, |acc, (signal, reading)| {
            let hashes = decipher_signal(signal);

            acc + decipher_reading(hashes, reading)
        })
}

fn decipher_reading(hashes: [usize; 10], reading: &str) -> usize {
    reading
        .trim()
        .split(' ')
        .rev()
        .enumerate()
        .map(|(idx, word)| (idx as u32, hash(word.trim())))
        .fold(0, |acc, (idx, hash)| {
            acc + hashes
                .iter()
                .enumerate()
                .find(|(_, val)| **val == hash)
                .unwrap()
                .0
                * 10usize.pow(idx)
        })
}

fn decipher_signal(signal: &str) -> [usize; 10] {
    let signal = signal.trim();
    let one = signal.split(' ').find(|word| word.len() == 2).unwrap();
    let four = signal.split(' ').find(|word| word.len() == 4).unwrap();

    let mut result = [0; 10];
    signal
        .split(' ')
        .map(|word| {
            let hash = hash(word);
            match word.len() {
                // known ones: 1, 4, 7, 8
                2 => (hash, 1),
                3 => (hash, 7),
                4 => (hash, 4),
                7 => (hash, 8),
                // 5 length ones: 2, 3, 5
                5 if mask_applies(one, word) => (hash, 3),
                5 if mask_overlap_amount(four, word) == 2 => (hash, 2),
                5 => (hash, 5),
                // 6 length ones: 0, 6, 9
                6 if mask_applies(four, word) => (hash, 9),
                6 if mask_applies(one, word) => (hash, 0),
                6 => (hash, 6),
                _ => panic!("oh no, word: {}", word),
            }
        })
        .for_each(|(hash, number)| result[number] = hash);

    result
}

fn get_prime(c: char) -> usize {
    match c {
        'a' => 2,
        'b' => 3,
        'c' => 5,
        'd' => 7,
        'e' => 11,
        'f' => 13,
        'g' => 17,
        _ => panic!("oh no, letter '{}'", c),
    }
}

fn hash(word: &str) -> usize {
    word.chars().map(get_prime).product()
}

// all characters of the mask is contained within the number
fn mask_applies(mask: &str, number: &str) -> bool {
    mask_overlap_amount(mask, number) == mask.len()
}

// returns the number of characters from the mask which are found within the number
fn mask_overlap_amount(mask: &str, number: &str) -> usize {
    mask.chars().filter(|c| number.contains(*c)).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task_1_passes() {
        let input = include_str!("test_input.txt");
        let data: Vec<&str> = input.lines().collect();

        assert_eq!(task_1(&data), 26);
    }

    #[test]
    fn task_2_passes() {
        let input = include_str!("test_input.txt");
        let data: Vec<&str> = input.lines().collect();

        assert_eq!(task_2(&data), 61229);
    }

    #[test]
    fn test_mask_overlap_amount() {
        [
            ("abc", "cba", 3),
            ("cda", "cba", 2),
            ("aghe", "ac", 1),
            ("a", "abc", 1),
            ("ab", "cd", 0),
        ]
        .iter()
        .for_each(|(mask, number, expected)| {
            assert_eq!(
                mask_overlap_amount(mask, number),
                *expected,
                "{} -> {}",
                mask,
                number
            );
        });
    }
    #[test]
    fn test_mask_applies() {
        [
            ("abc", "cba", true),
            ("cda", "cba", false),
            ("aghe", "ac", false),
            ("a", "abc", true),
            ("ab", "cd", false),
            ("cf", "acdfg", true),
        ]
        .iter()
        .for_each(|(mask, number, expected)| {
            assert_eq!(
                mask_applies(mask, number),
                *expected,
                "{} -> {}",
                mask,
                number
            );
        });
    }
}
