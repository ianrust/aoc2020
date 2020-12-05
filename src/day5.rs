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

fn bit_cast(bits: &[bool]) -> u8 {
    bits.iter()
        .fold(0, |byte: u8, &bit: &bool| (byte << 1) + (bit as u8))
}

fn to_id_iter(input: &Vec<SeatSpec>) -> impl Iterator<Item = u32> + '_ {
    input.into_iter().map(|spec| {
        let row = bit_cast(&spec.row_bits);
        let col = bit_cast(&spec.col_bits);
        (row as u32 * 8) + col as u32
    })
}

#[aoc(day5, part1)]
pub fn part1(input: &Vec<SeatSpec>) -> u32 {
    to_id_iter(input).max().unwrap() as u32
}

#[aoc(day5, part2)]
pub fn part2(input: &Vec<SeatSpec>) -> u32 {
    let mut ids = to_id_iter(input).collect::<Vec<u32>>();
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
