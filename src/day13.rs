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

fn find_factor_slow(ids: &[(usize, &u128)]) -> u128 {
    let first = ids[0];
    println!("last , {:?}, {:?}", first, ids);
    let mut trial_earliest = *first.1 - first.0 as u128;
    while !test_waits(trial_earliest, &ids) {
        trial_earliest += *first.1;
    }
    trial_earliest
}

fn find_factor(mult_factor: u128, first_elem_mult: u128, ids: &[(usize, &u128)]) -> (u128, u128) {
    let second_to_last = ids[ids.len() - 1];
    let mut trial_earliest = (first_elem_mult + 1) * ids[0].1 - ids[0].0 as u128;
    while !test_waits(trial_earliest, &ids) {
        trial_earliest += ids[0].1;
    }
    println!(
        "trial earliest: {}, mult_factor: {}, second_to_last: {:?}, {}",
        trial_earliest,
        mult_factor,
        second_to_last,
        (trial_earliest + second_to_last.0 as u128) / second_to_last.1
    );
    (
        (trial_earliest + second_to_last.0 as u128) / second_to_last.1 / mult_factor,
        (trial_earliest + ids[0].0 as u128 - ids[0].1) / ids[0].1,
    )
}

#[aoc(day13, part2)]
pub fn part2(notes: &Notes) -> u128 {
    let mut id_sorted = notes
        .bus_ids
        .iter()
        .enumerate()
        .collect::<Vec<(usize, &u128)>>();
    id_sorted.sort_by(|a, b| (b.1).partial_cmp(&a.1).expect("bad comparison"));
    // remove zeroes
    id_sorted.retain(|a| *a.1 != 0u128);
    let mut mult_factor = 1;
    let mut first_elem_mult = 0;
    for l in 1..id_sorted.len() + 1 {
        println!(
            "mult_factor: {:?}, l: {}, len: {}",
            mult_factor,
            l,
            id_sorted.len()
        );
        let factor = find_factor(mult_factor, first_elem_mult, &id_sorted[0..l]);
        mult_factor = factor.0;
        first_elem_mult = factor.1;
        let slow_factor = find_factor_slow(&id_sorted[0..l]);
        println!("slow factor, {}", slow_factor,);
        println!("{}, {}, {}", l, id_sorted.len(), mult_factor);
    }
    (first_elem_mult + 1) * id_sorted[0].1 - id_sorted[0].0 as u128 + 1
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
