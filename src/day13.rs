#[derive(Debug)]
pub struct Notes {
    earliest: u128,
    bus_ids: Vec<u128>,
}

impl Notes {
    pub fn as_ref(&self) -> &Self {
        self
    }
}

#[aoc_generator(day13)]
pub fn input_generator(input: &str) -> Notes {
    let mut lines_iter = input.lines();
    Notes {
        earliest: lines_iter
            .next()
            .expect("panic indexing earliest time")
            .trim()
            .parse::<u128>()
            .expect("panic parsing earliest time"),
        bus_ids: lines_iter
            .next()
            .expect("panic indexing earliest time")
            .trim()
            .split(',')
            .fold(Vec::<u128>::new(), |mut bus_ids, id| {
                if !id.trim().eq("x") {
                    bus_ids.push(id.parse::<u128>().expect("error parsing id"));
                } else {
                    // 0 means x
                    bus_ids.push(0);
                }
                bus_ids
            }),
    }
}

#[aoc(day13, part1)]
pub fn part1(notes: &Notes) -> u128 {
    let min_wait_result =
        notes
            .bus_ids
            .iter()
            .fold((std::u128::MAX, 0u128), |min_wait_res, bus_id| {
                if bus_id != &0u128 {
                    let time_after = bus_id - (notes.earliest % bus_id);
                    if time_after < min_wait_res.0 {
                        return (time_after, *bus_id);
                    }
                }
                min_wait_res
            });
    min_wait_result.0 * min_wait_result.1
}

fn test_waits(test_timestamp: u128, ids: &[(usize, &u128)]) -> bool {
    for id in ids {
        if *id.1 == 0 {
            continue;
        } else if (test_timestamp + id.0 as u128) % id.1 != 0 {
            return false;
        }
    }
    true
}

fn find_factor(step: u128, start: u128, ids: &[(usize, &u128)]) -> u128 {
    let mut trial_earliest = start;
    while !test_waits(trial_earliest, &ids) {
        trial_earliest += step;
    }
    trial_earliest
}

#[aoc(day13, part2)]
pub fn part2(notes: &Notes) -> u128 {
    let mut id_sorted = notes
        .bus_ids
        .iter()
        .enumerate()
        .collect::<Vec<(usize, &u128)>>();
    // remove zeroes
    id_sorted.retain(|a| *a.1 != 0u128);
    let mut step = 1;
    let mut start = *id_sorted[0].1;
    for l in 1..id_sorted.len() + 1 {
        start = find_factor(step, start, &id_sorted[0..l]);
        step = find_factor(step, start + step, &id_sorted[0..l]) - start;
    }

    start
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "939
                          7,13,x,x,59,x,31,19";

    #[test]
    fn sample1() {
        assert_eq!(part1(&input_generator(SAMPLE)), 295);
    }

    #[test]
    fn sample2() {
        assert_eq!(part2(&input_generator(SAMPLE)), 1068781);
    }
}
