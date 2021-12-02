pub fn task_1(input: &[&str]) -> i32 {
    let (v, h) = input
        .iter()
        .map(|line| line.split_ascii_whitespace().collect())
        .map(|parts: Vec<&str>| (parts[0], parts[1].parse::<i32>().unwrap()))
        .fold((0, 0), |acc, (instruction, amount)| match instruction {
            "forward" => (acc.0, acc.1 + amount),
            "up" => (acc.0 - amount, acc.1),
            "down" => (acc.0 + amount, acc.1),
            _ => panic!("oh no"),
        });

    v * h
}

struct Position {
    v: i32,
    h: i32,
    a: i32,
}

impl Position {
    fn new() -> Self {
        Self { v: 0, h: 0, a: 0 }
    }

    fn forward(&self, amount: i32) -> Self {
        Self {
            v: self.v + self.a * amount,
            h: self.h + amount,
            a: self.a,
        }
    }

    fn up(&self, amount: i32) -> Self {
        Self {
            v: self.v,
            h: self.h,
            a: self.a - amount,
        }
    }

    fn down(&self, amount: i32) -> Self {
        Self {
            v: self.v,
            h: self.h,
            a: self.a + amount,
        }
    }

    fn compute(&self) -> i32 {
        self.v * self.h
    }
}

pub fn task_2(input: &[&str]) -> i32 {
    let pos = input
        .iter()
        .map(|line| line.split_ascii_whitespace().collect())
        .map(|parts: Vec<&str>| (parts[0], parts[1].parse::<i32>().unwrap()))
        .fold(
            Position::new(),
            |pos, (instruction, amount)| match instruction {
                "forward" => pos.forward(amount),
                "up" => pos.up(amount),
                "down" => pos.down(amount),
                _ => panic!("oh no"),
            },
        );

    pos.compute()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task_1_passes() {
        let data = vec![
            "forward 5",
            "down 5",
            "forward 8",
            "up 3",
            "down 8",
            "forward 2",
        ];

        assert_eq!(task_1(&data), 150);
    }

    #[test]
    fn task_2_passes() {
        let data = vec![
            "forward 5",
            "down 5",
            "forward 8",
            "up 3",
            "down 8",
            "forward 2",
        ];

        assert_eq!(task_2(&data), 900);
    }
}
