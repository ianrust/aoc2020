use regex::Regex;
use std::collections::HashMap;
use std::rc::Rc;

#[aoc_generator(day7)]
pub fn input_generator(
    input: &str,
) -> Rc<(
    HashMap<String, HashMap<String, u32>>,
    HashMap<String, HashMap<String, u32>>,
)> {
    let re_qty = Regex::new(r"( bags, )|( bag, )|( bags.)|( bag.)").unwrap();
    let mut may_be_held_by: HashMap<String, HashMap<String, u32>> = HashMap::new();
    let mut holdings: HashMap<String, HashMap<String, u32>> = HashMap::new();
    for l in input.lines() {
        let big_split = l.split(" bags contain ").collect::<Vec<&str>>();

        let qty_strings = re_qty.split(big_split[1]);

        for qty_string in qty_strings {
            let mut qty_split = qty_string.trim().splitn(2, ' ');
            let number_str = qty_split.next().unwrap().trim();
            if number_str.eq("no") || number_str.eq("") {
                break;
            }
            let number = number_str.parse::<u32>().unwrap();
            let color = String::from(qty_split.next().unwrap().trim());

            let big_color = String::from(big_split[0].trim());
            if let Some(holders) = may_be_held_by.get_mut(&color) {
                holders.insert(big_color.clone(), number);
            } else {
                let mut holders = HashMap::<String, u32>::new();
                holders.insert(big_color.clone(), number);
                may_be_held_by.insert(color.clone(), holders);
            }

            // also do holdings
            if let Some(insides) = holdings.get_mut(&(big_color.clone())) {
                insides.insert(color, number);
            } else {
                let mut insides = HashMap::<String, u32>::new();
                insides.insert(color.clone(), number);
                holdings.insert(big_color.clone(), insides);
            }
        }
    }

    Rc::new((may_be_held_by, holdings))
}

fn get_num_bags(holdings: &HashMap<String, HashMap<String, u32>>, top: &String) -> u32 {
    if let Some(holders) = holdings.get(top) {
        let mut count: u32 = 0;
        for (holder, qty) in holders {
            let count_below = get_num_bags(holdings, holder);
            count += qty * (1 + count_below);
        }
        count
    } else {
        0
    }
}

fn get_children(
    may_be_held_by: &HashMap<String, HashMap<String, u32>>,
    top: &String,
    root: bool,
) -> Vec<String> {
    if let Some(holders) = may_be_held_by.get(top) {
        let mut children = Vec::<String>::new();
        for (holder, _) in holders {
            let new_children = get_children(may_be_held_by, holder, false);
            if !root {
                children.push(top.clone());
            }
            children.extend(new_children);
        }
        children
    } else {
        vec![top.clone()]
    }
}

fn get_unique_children(
    may_be_held_by: &HashMap<String, HashMap<String, u32>>,
    top: &String,
) -> Vec<String> {
    let mut children = get_children(may_be_held_by, top, true);
    children.sort();
    children.dedup();
    children
}

#[aoc(day7, part1)]
pub fn part1(
    input: &(
        HashMap<String, HashMap<String, u32>>,
        HashMap<String, HashMap<String, u32>>,
    ),
) -> u32 {
    let unique_children = get_unique_children(&input.0, &String::from("shiny gold"));
    unique_children.len() as u32
}

#[aoc(day7, part2)]
pub fn part2(
    input: &(
        HashMap<String, HashMap<String, u32>>,
        HashMap<String, HashMap<String, u32>>,
    ),
) -> u32 {
    get_num_bags(&input.1, &String::from("shiny gold"))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLEA: &str = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

    const SAMPLEB: &str = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";

    #[test]
    fn sample1() {
        assert_eq!(part1(&input_generator(SAMPLEA)), 4)
    }

    #[test]
    fn sample2() {
        assert_eq!(part2(&input_generator(SAMPLEA)), 32)
    }

    #[test]
    fn sample3() {
        assert_eq!(part2(&input_generator(SAMPLEB)), 126)
    }
}
