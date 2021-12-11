type Consortium = [u32; 100];
type Flashed = [State; 100];
type Direction = (i32, i32);

#[derive(Copy, Clone, PartialEq)]
enum State {
    Flashed,
    NeedIncrease,
    Normal,
}

const ALL_DIRECTIONS: [Direction; 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

pub fn task_1(input: &str) -> usize {
    let mut consortium: Consortium = [0; 100];

    input
        .chars()
        .enumerate()
        .for_each(|(idx, c)| consortium[idx] = c.to_digit(10).unwrap());

    (0..100).fold(0, |acc, _| acc + step(&mut consortium))
}

fn step(c: &mut Consortium) -> usize {
    fn calc_flashes(c: &mut Consortium, f: &mut Flashed) -> usize {
        let need_flash = c
            .iter()
            .enumerate()
            .filter(|(idx, _)| f[*idx] == State::Normal && c[*idx] > 9)
            .count();

        if need_flash == 0 {
            return 0;
        }

        do_flash(c, f);

        need_flash + calc_flashes(c, f)
    }

    fn do_flash(c: &mut Consortium, f: &mut Flashed) {
        for (_, state) in f
            .iter_mut()
            .enumerate()
            .filter(|(idx, state)| *state == &State::Normal && c[*idx] > 9)
        {
            *state = State::NeedIncrease;
        }

        for (idx, state) in f
            .iter_mut()
            .enumerate()
            .filter(|(_, state)| *state == &State::NeedIncrease)
        {
            increase_neighbours(c, idx);
            *state = State::Flashed;
        }
    }

    fn increase_neighbours(c: &mut Consortium, idx: usize) {
        ALL_DIRECTIONS
            .iter()
            .filter_map(|dir| get_idx(idx as i32, dir))
            .for_each(|idx| c[idx] += 1);
    }

    let mut flashes = [State::Normal; 100];
    (0..100).for_each(|idx| c[idx] += 1);

    let flash_count = calc_flashes(c, &mut flashes);

    c.iter_mut().filter(|o| **o > 9).for_each(|o| *o = 0);

    flash_count
}

fn get_idx(current: i32, dir: &Direction) -> Option<usize> {
    match (current, dir) {
        (c, (y, _)) if c < 10 && *y < 0 => None,      // top overflow
        (c, (y, _)) if c > 89 && *y > 0 => None,      // bottom overflow
        (c, (_, x)) if c % 10 == 9 && *x > 0 => None, // right overflow
        (c, (_, x)) if c % 10 == 0 && *x < 0 => None, // left overflow
        (c, (y, x)) => Some((c + x + y * 10).try_into().unwrap()),
    }
}

pub fn task_2(input: &str) -> usize {
    let mut consortium: Consortium = [0; 100];

    input
        .chars()
        .enumerate()
        .for_each(|(idx, c)| consortium[idx] = c.to_digit(10).unwrap());

    for i in 0.. {
        if step(&mut consortium) == 100 {
            return i + 1;
        }
    }

    panic!("hehen't");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task_1_passes() {
        let data = include_str!("test_input.txt")
            .lines()
            .collect::<Vec<&str>>()
            .join("");

        assert_eq!(task_1(&data), 1656);
    }

    #[test]
    fn task_2_passes() {
        let data = include_str!("test_input.txt")
            .lines()
            .collect::<Vec<&str>>()
            .join("");

        assert_eq!(task_2(&data), 195);
    }
}
