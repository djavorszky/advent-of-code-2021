type Consortium = [u32; 100];
type Flashed = [bool; 100];
type Direction = (i32, i32);

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
        let flashing = c
            .iter()
            .enumerate()
            .filter(|(idx, _)| !f[*idx] && c[*idx] > 9)
            .count();

        if flashing == 0 {
            return 0;
        }

        do_flash(c, f);

        flashing + calc_flashes(c, f)
    }

    fn do_flash(c: &mut Consortium, f: &mut Flashed) {
        let need_to_flash: Vec<usize> = c
            .iter()
            .enumerate()
            .filter(|(idx, _)| !f[*idx] && c[*idx] > 9)
            .map(|(idx, _)| idx)
            .collect();

        need_to_flash.into_iter().for_each(|idx| {
            f[idx] = true;
            increase_neighbours(c, idx);
        });
    }

    fn increase_neighbours(c: &mut Consortium, idx: usize) {
        ALL_DIRECTIONS
            .iter()
            .filter_map(|dir| get_idx(idx as i32, dir))
            .for_each(|idx| c[idx] = c[idx] + 1);
    }

    let mut flashes = [false; 100];
    (0..100).for_each(|idx| c[idx] = c[idx] + 1);

    let flash_count = calc_flashes(c, &mut flashes);

    for i in 0..100 {
        if c[i] > 9 {
            c[i] = 0
        }
    }

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
