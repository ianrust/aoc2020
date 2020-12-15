#[aoc_generator(day15)]
pub fn input_generator(input: &str) -> Vec<u32> {
    input
        .split(",")
        .map(|n| n.trim().parse::<u32>().expect("unable to parse number"))
        .collect::<Vec<u32>>()
}

fn insert_expand(turn_last_spoken: &mut Vec<u32>, number: u32, turn: u32) {
    // println!(
    //     "{}, {}, {}",
    //     (number as usize),
    //     turn_last_spoken.len(),
    //     (number as usize) > turn_last_spoken.len()
    // );
    if (number as usize) >= turn_last_spoken.len() {
        turn_last_spoken.resize(number as usize + 1, 0);
    }
    turn_last_spoken[number as usize] = turn;
}

fn get_final_number(numbers: &Vec<u32>, until: u32) -> u32 {
    // key: number, value: turn
    let mut turn_last_spoken = Vec::<u32>::new();
    // populate beginning
    for (turn_zero, num) in numbers[0..(numbers.len() - 1 as usize)].iter().enumerate() {
        insert_expand(&mut turn_last_spoken, *num, turn_zero as u32 + 1);
    }

    // play game
    // println!("{:?} numbers", numbers);
    let mut last_number: u32 = *numbers.last().unwrap();
    for turn in (numbers.len() as u32)..until {
        let number: u32;
        match turn_last_spoken.get(last_number as usize) {
            Some(last_turn_spoken) => {
                if last_turn_spoken != &0 {
                    number = turn - last_turn_spoken;
                } else {
                    number = 0;
                }
            }
            None => number = 0,
        }
        insert_expand(&mut turn_last_spoken, last_number, turn);
        last_number = number;
    }

    last_number
}

#[aoc(day15, part1)]
pub fn part1(numbers: &Vec<u32>) -> u32 {
    get_final_number(numbers, 2020)
}

#[aoc(day15, part2)]
pub fn part2(numbers: &Vec<u32>) -> u32 {
    get_final_number(numbers, 30000000)
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLES: [(&str, u32, u32); 7] = [
        ("0,3,6", 436, 175594),
        ("1,3,2", 1, 2578),
        ("2,1,3", 10, 3544142),
        ("1,2,3", 27, 261214),
        ("2,3,1", 78, 6895259),
        ("3,2,1", 438, 18),
        ("3,1,2", 1836, 362),
    ];

    #[test]
    fn sample1() {
        for sample in SAMPLES.iter() {
            assert_eq!(part1(&input_generator(sample.0)), sample.1);
        }
    }

    #[test]
    fn sample2() {
        for sample in SAMPLES.iter() {
            assert_eq!(part2(&input_generator(sample.0)), sample.2);
        }
    }
}
