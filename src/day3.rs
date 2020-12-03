struct Slope {
    right: usize,
    down: usize,
}

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<Vec<bool>> {
    input.lines().map(|l| {
        l.trim().chars().map(|c| {
            c == '#'
        }).collect()
    }).collect()
}

fn count_trees(input: &Vec<Vec<bool>>, slope: &Slope) -> u32 {
    let mut count = 0;
    let mut step: usize = 0;
    let width = input[0].len();
    let height = input.len();
    let mut col_index;
    let mut row_index = 0;
    while row_index < height {
        step += 1;
        col_index = (step * slope.right) % width;
        row_index = step * slope.down;

        if row_index >= height {
            break;
        }

        if input[row_index][col_index] {
            count += 1;
        }
    }
    count
}

#[aoc(day3, part1)]
pub fn part1(input: &Vec<Vec<bool>>) -> u32 {
    let slope = Slope{
        right: 3,
        down: 1,
    };
    count_trees(input, &slope)
}

#[aoc(day3, part2)]
pub fn part2(input: &Vec<Vec<bool>>) -> u32 {
    let slopes = vec![Slope{
        right: 1,
        down: 1,
    }, Slope{
        right: 3,
        down: 1,
    }, Slope{
        right: 5,
        down: 1,
    }, Slope{
        right: 7,
        down: 1,
    }, Slope{
        right: 1,
        down: 2,
    }];

    slopes.iter().map(|slope| {
        count_trees(input, &slope)
    }).collect::<Vec<u32>>().iter().product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let sample =   "..##.......
                        #...#...#..
                        .#....#..#.
                        ..#.#...#.#
                        .#...##..#.
                        ..#.##.....
                        .#.#.#....#
                        .#........#
                        #.##...#...
                        #...##....#
                        .#..#...#.#";
        assert_eq!(part1(&input_generator(sample)), 7);
    }

    #[test]
    fn sample2() {
        let sample =   "..##.......
                        #...#...#..
                        .#....#..#.
                        ..#.#...#.#
                        .#...##..#.
                        ..#.##.....
                        .#.#.#....#
                        .#........#
                        #.##...#...
                        #...##....#
                        .#..#...#.#";
        assert_eq!(part2(&input_generator(sample)), 336);
    }
}