#[derive(Debug)]
pub struct Direction {
    action: char,
    value: i32,
}

#[derive(Debug)]
struct Position {
    x: f64,
    y: f64,
    orientation: i32, //direction in degrees, 0 -> east
}

impl Position {
    pub fn new() -> Position {
        Position {
            x: 0f64,
            y: 0f64,
            orientation: 0,
        }
    }

    pub fn add(&self, to: &Position) -> Position {
        Position {
            x: (to.x + self.x).round(),
            y: (to.y + self.y).round(),
            orientation: 0,
        }
    }

    pub fn mul(&self, multiple: i32) -> Position {
        Position {
            x: self.x * multiple as f64,
            y: self.y * multiple as f64,
            orientation: 0,
        }
    }

    pub fn rotate(&self, angle: f64) -> Position {
        Position {
            x: (angle.cos() * self.x - angle.sin() * self.y).round(),
            y: (angle.sin() * self.x + angle.cos() * self.y).round(),
            orientation: 0,
        }
    }
}

#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> Vec<Direction> {
    input
        .lines()
        .map(|l| {
            let line = l.trim();
            Direction {
                action: line.chars().next().expect("couldn't get first char"),
                value: line[1..].parse::<i32>().expect("invalid value to parse"),
            }
        })
        .collect::<Vec<Direction>>()
}

fn run_directions(directions: &Vec<Direction>) -> Position {
    let mut current_pos = Position::new();

    for direction in directions {
        match direction.action {
            'N' => current_pos.y += direction.value as f64,
            'S' => current_pos.y -= direction.value as f64,
            'E' => current_pos.x += direction.value as f64,
            'W' => current_pos.x -= direction.value as f64,
            'L' => current_pos.orientation += direction.value,
            'R' => current_pos.orientation -= direction.value,
            'F' => {
                let dir_rads = (current_pos.orientation as f64).to_radians();
                current_pos.x += (dir_rads.cos() * direction.value as f64).round();
                current_pos.y += (dir_rads.sin() * direction.value as f64).round();
            }
            error => panic!("invalid direction: {}", error),
        }
    }

    current_pos
}

fn run_waypoint_directions(directions: &Vec<Direction>) -> Position {
    let mut current_pos = Position::new();
    let mut waypoint = Position {
        x: 10f64,
        y: 1f64,
        orientation: 0,
    };

    for direction in directions {
        match direction.action {
            'N' => waypoint.y += direction.value as f64,
            'S' => waypoint.y -= direction.value as f64,
            'E' => waypoint.x += direction.value as f64,
            'W' => waypoint.x -= direction.value as f64,
            'L' => {
                let value_rads = (direction.value as f64).to_radians();
                waypoint = waypoint.rotate(value_rads);
            }
            'R' => {
                let value_rads = (-direction.value as f64).to_radians();
                waypoint = waypoint.rotate(value_rads);
            }
            'F' => {
                current_pos = current_pos.add(&waypoint.mul(direction.value));
            }
            error => panic!("invalid direction: {}", error),
        }
    }

    current_pos
}

#[aoc(day12, part1)]
pub fn part1(directions: &Vec<Direction>) -> u32 {
    let final_pos = run_directions(directions);
    final_pos.x.abs() as u32 + final_pos.y.abs() as u32
}

#[aoc(day12, part2)]
pub fn part2(directions: &Vec<Direction>) -> u32 {
    let final_pos = run_waypoint_directions(directions);
    final_pos.x.abs() as u32 + final_pos.y.abs() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "F10
N3
F7
R90
F11";

    #[test]
    fn sample1() {
        assert_eq!(part1(&input_generator(SAMPLE)), 25);
    }

    #[test]
    fn sample2() {
        assert_eq!(part2(&input_generator(SAMPLE)), 286);
    }
}
