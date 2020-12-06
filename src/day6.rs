use std::collections::HashMap;

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Vec<Vec<String>> {
    input
        .split("\n\n")
        .map(|g| {
            g.trim()
                .lines()
                .map(|p| String::from(p.trim()))
                .collect::<Vec<String>>()
        })
        .collect::<Vec<Vec<String>>>()
}

#[aoc(day6, part1)]
pub fn part1(input: &Vec<Vec<String>>) -> u32 {
    input
        .iter()
        .map(|g| {
            let mut all_qs = g.join("").trim().chars().collect::<Vec<char>>();
            all_qs.sort();
            all_qs.dedup();
            all_qs.len() as u32
        })
        .sum::<u32>()
}

#[aoc(day6, part2)]
pub fn part2(input: &Vec<Vec<String>>) -> u32 {
    input
        .iter()
        .map(|g| {
            let num_people = g.len();
            let mut counts = HashMap::new();

            for q in g.join("").trim().chars() {
                let counter = counts.entry(q).or_insert(0);
                *counter += 1;
            }

            counts
                .iter()
                .map(|(_, num)| if num == &num_people { 1 } else { 0 })
                .sum::<u32>()
        })
        .sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE: &str = "abc

    a
    b
    c

    ab
    ac

    a
    a
    a
    a

    b";

    #[test]
    fn sample1() {
        assert_eq!(part1(&input_generator(SAMPLE)), 11);
    }

    #[test]
    fn sample2() {
        assert_eq!(part2(&input_generator(SAMPLE)), 6);
    }
}
