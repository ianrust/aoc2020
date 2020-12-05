use std::convert::TryFrom;

const RS: usize = 7;
const CS: usize = 3;

pub struct SeatSpec {
    row_bits: [bool; RS],
    col_bits: [bool; CS],
}

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Vec<SeatSpec> {
    input
        .lines()
        .map(|l| {
            let mut spec = SeatSpec {
                row_bits: [false; RS],
                col_bits: [false; CS],
            };
            let mut char_iter = l.trim().chars();
            for i in 0..RS {
                spec.row_bits[i] = char_iter.next().unwrap() == 'B';
            }
            for i in 0..CS {
                spec.col_bits[i] = char_iter.next().unwrap() == 'R';
            }
            spec
        })
        .collect::<Vec<SeatSpec>>()
}

fn bit_cast(bits: &[bool], msb_pos: usize) -> u8 {
    let mut bits_iter = bits.iter().enumerate();
    let mut byte: u8 = 0;
    while let Some((pos, &bit)) = bits_iter.next() {
        if bit {
            let pos_value = 1 << u32::try_from(msb_pos - 1 - pos).unwrap();
            byte += pos_value;
        }
    }
    byte
}

#[aoc(day5, part1)]
pub fn part1(input: &Vec<SeatSpec>) -> u32 {
    input
        .into_iter()
        .map(|spec| {
            let row = bit_cast(&spec.row_bits, RS);
            let col = bit_cast(&spec.col_bits, CS);
            (row as u32 * 8) + col as u32
        })
        .max()
        .unwrap() as u32
}

#[aoc(day5, part2)]
pub fn part2(input: &Vec<SeatSpec>) -> u32 {
    let mut ids = input
        .into_iter()
        .map(|spec| {
            let row = bit_cast(&spec.row_bits, RS);
            let col = bit_cast(&spec.col_bits, CS);
            (row as u32 * 8) + col as u32
        })
        .collect::<Vec<u32>>();
    ids.sort();
    let mut id = 0;
    for id_window in ids.windows(2) {
        if id_window[1] - id_window[0] == 2 {
            id = id_window[0] + 1;
            break;
        }
    }
    id
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sample0() {
        let sample = "FBFBBFFRLR";
        assert_eq!(part1(&input_generator(sample)), 357);
    }

    #[test]
    fn sample1() {
        let sample = "BFFFBBFRRR";
        assert_eq!(part1(&input_generator(sample)), 567);
    }

    #[test]
    fn sample2() {
        let sample = "FFFBBBFRRR";
        assert_eq!(part1(&input_generator(sample)), 119);
    }

    #[test]
    fn sample3() {
        let sample = "BBFFBBFRLL";
        assert_eq!(part1(&input_generator(sample)), 820);
    }

    #[test]
    fn sample_comb() {
        let sample = "BFFFBBFRRR\nFFFBBBFRRR\nBBFFBBFRLL";
        assert_eq!(part1(&input_generator(sample)), 820);
    }
}
