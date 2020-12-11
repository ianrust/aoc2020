#[derive(Clone, Eq, PartialEq)]
pub enum SeatState {
    EMPTY,
    OCCUPIED,
    FLOOR,
}

const DIRECTIONS: [(i64, i64); 8] = [
    (0, 1),
    (1, 1),
    (1, 0),
    (1, -1),
    (0, -1),
    (-1, -1),
    (-1, 0),
    (-1, 1),
];

#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> Vec<Vec<SeatState>> {
    input
        .lines()
        .map(|l| {
            l.trim()
                .chars()
                .map(|c| match c {
                    'L' => SeatState::EMPTY,
                    '#' => SeatState::OCCUPIED,
                    '.' => SeatState::FLOOR,
                    _ => panic!("invalid code"),
                })
                .collect::<Vec<SeatState>>()
        })
        .collect::<Vec<Vec<SeatState>>>()
}

fn valid(dir: &(i64, i64), r: usize, c: usize, h: usize, w: usize) -> bool {
    let pos = (dir.0 + r as i64, dir.1 + c as i64);
    pos.0 >= 0 && pos.0 <= (h as i64 - 1) && pos.1 >= 0 && pos.1 <= (w as i64 - 1)
}

fn step_sight(map: &mut Vec<Vec<SeatState>>) {
    let height = map.len();
    let width = map[0].len();
    let mut new_occs = Vec::<(usize, usize)>::new();
    let mut new_emps = Vec::<(usize, usize)>::new();
    for r in 0..height {
        for c in 0..width {
            let mut stopped = vec![false; 8];
            let mut step = 1;
            let mut occ_count: u8 = 0;
            loop {
                // spiral out
                let mut iterator = DIRECTIONS.iter().enumerate();
                while let Some((i, d)) = iterator.next() {
                    // circle
                    let scaled_dir = (d.0 * step, d.1 * step);
                    let v = valid(&scaled_dir, r, c, height, width);
                    if v && !stopped[i] {
                        let pos = (
                            (scaled_dir.0 + r as i64) as usize,
                            (scaled_dir.1 + c as i64) as usize,
                        );
                        match map[pos.0][pos.1] {
                            SeatState::OCCUPIED => {
                                occ_count += 1;
                                stopped[i] = true
                            }
                            SeatState::EMPTY => stopped[i] = true,
                            SeatState::FLOOR => {}
                        }
                    } else {
                        stopped[i] = true;
                    }
                }

                step += 1;

                if stopped.iter().fold(true, |cum, s| cum && *s) {
                    break;
                }
            }
            match map[r][c] {
                SeatState::OCCUPIED => {
                    if occ_count >= 5 {
                        new_emps.push((r, c));
                    }
                }
                SeatState::EMPTY => {
                    if occ_count == 0 {
                        new_occs.push((r, c));
                    }
                }
                _ => {}
            }
        }
    }

    for (r, c) in new_emps {
        map[r][c] = SeatState::EMPTY;
    }

    for (r, c) in new_occs {
        map[r][c] = SeatState::OCCUPIED;
    }
}

fn step(map: &Vec<Vec<SeatState>>) -> Vec<Vec<SeatState>> {
    let mut new_map = map.clone();
    let height = map.len();
    let width = map[0].len();
    for r in 0..height {
        for c in 0..width {
            let occ_count: u8 = DIRECTIONS.iter().fold(0u8, |occ, dir| {
                let pos = ((dir.0 + r as i64) as usize, (dir.1 + c as i64) as usize);
                if valid(dir, r, c, height, width) && map[pos.0][pos.1] == SeatState::OCCUPIED {
                    occ + 1
                } else {
                    occ
                }
            });

            match map[r][c] {
                SeatState::OCCUPIED => {
                    if occ_count >= 4 {
                        new_map[r][c] = SeatState::EMPTY
                    }
                }
                SeatState::EMPTY => {
                    if occ_count == 0 {
                        new_map[r][c] = SeatState::OCCUPIED
                    }
                }
                _ => {}
            }
        }
    }

    new_map
}

fn count_occ(map: &Vec<Vec<SeatState>>) -> u32 {
    map.iter().flatten().fold(0u32, |num_occ, state| {
        if state == &SeatState::OCCUPIED {
            num_occ + 1
        } else {
            num_occ
        }
    })
}

#[aoc(day11, part1)]
pub fn part1(map: &Vec<Vec<SeatState>>) -> u32 {
    let mut territory = map.clone();
    let mut last_occ_count: i64 = -1;
    loop {
        territory = step(&territory);
        let this_occ_count = count_occ(&territory) as i64;
        if last_occ_count == this_occ_count {
            return this_occ_count as u32;
        }
        last_occ_count = this_occ_count;
    }
}

#[aoc(day11, part2)]
pub fn part2(map: &Vec<Vec<SeatState>>) -> u32 {
    let mut territory = map.clone();
    let mut last_occ_count: i64 = -1;
    loop {
        step_sight(&mut territory);
        let this_occ_count = count_occ(&territory) as i64;
        if last_occ_count == this_occ_count {
            return this_occ_count as u32;
        }
        last_occ_count = this_occ_count;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let sample = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";
        assert_eq!(part1(&input_generator(sample)), 37);
    }

    #[test]
    fn sample2() {
        let sample = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";
        assert_eq!(part2(&input_generator(sample)), 26);
    }
}
