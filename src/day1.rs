type Gift = (u32, u32, u32);

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<Gift> {
    input
        .lines()
        .map(|l| {
            let mut gift = l.trim().split('x').map(|d| d.parse().unwrap());
            (
                gift.next().unwrap(),
                gift.next().unwrap(),
                gift.next().unwrap(),
            )
        }).collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[Gift]) -> u32 {
    input
        .iter()
        .map(|&(l, w, h)| {
            let (s1, s2) = smallest_side((l, w, h));

            2 * l * w + 2 * w * h + 2 * h * l + s1 * s2
        }).sum()
}