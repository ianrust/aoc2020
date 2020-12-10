#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Vec<u64> {
    let mut adapters = input
        .lines()
        .map(|l| l.trim().parse::<u64>().expect("Didn't get a number"))
        .collect::<Vec<u64>>();

    adapters.push(0);
    adapters.push(adapters.iter().max().expect("failed to get max") + 3);
    adapters.sort();
    adapters
}

#[aoc(day10, part1)]
pub fn part1(adapters: &Vec<u64>) -> u64 {
    let mut distribution: [u64; 4] = [0; 4];
    for window in adapters.windows(2) {
        let index = (window[1] as i32 - window[0] as i32).abs();
        distribution[index as usize] += 1;
    }
    distribution[1] * distribution[3]
}

fn fib_sum(index: u64) -> u64 {
    let mut fib0 = 0;
    let mut fib1 = 1;
    let mut sum = fib1;
    for _ in 1..index {
        let fib1_temp = fib1;
        fib1 = fib0 + fib1;
        fib0 = fib1_temp;
        sum += fib0;
    }
    sum
}

struct LazyFibSum {
    lookup: Vec<u64>,
}

impl LazyFibSum {
    pub fn new() -> LazyFibSum {
        LazyFibSum {
            lookup: Vec::<u64>::new(),
        }
    }
    pub fn get(&mut self, index: u64) -> u64 {
        let offseted_index = index + 1;
        match self.lookup.get(offseted_index as usize) {
            Some(sum) => return sum.clone(),
            _ => {
                for i in (self.lookup.len() as u64)..(offseted_index + 1) {
                    self.lookup.push(fib_sum(i) - 1);
                }
                return self.lookup[self.lookup.len() - 1];
            }
        }
    }
}

#[aoc(day10, part2)]
pub fn part2(adapters: &Vec<u64>) -> u64 {
    let mut fibsum = LazyFibSum::new();
    let diffs = adapters
        .windows(2)
        .map(|w| (w[1] as i32 - w[0] as i32).abs() as u64)
        .collect::<Vec<u64>>();
    // count each stretch of ones, convert to number of combinations that can occur for that group
    let mut one_combos = Vec::<u64>::new();
    let mut one_count = 0;
    for diff in diffs.iter() {
        match diff {
            1 => one_count += 1,
            _ => {
                let combos = fibsum.get(one_count);
                if combos != 0 {
                    one_combos.push(combos);
                }
                one_count = 0;
            }
        }
    }
    one_combos.iter().product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let sample = "16
10
15
5
1
11
7
19
6
12
4";
        assert_eq!(part1(&input_generator(sample)), 7 * 5);
        assert_eq!(part2(&input_generator(sample)), 8);
    }

    #[test]
    fn sample2() {
        let sample = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";
        assert_eq!(part1(&input_generator(sample)), 22 * 10);
        assert_eq!(part2(&input_generator(sample)), 19208);
    }
}
