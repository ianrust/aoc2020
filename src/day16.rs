use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Rule {
    field: String,
    ranges: Vec<(u32, u32)>,
}

#[derive(Debug)]
pub struct Notes {
    rules: Vec<Rule>,
    my_ticket: Vec<u32>,
    nearby_tickets: Vec<Vec<u32>>,
}

impl Notes {
    pub fn as_ref(&self) -> &Self {
        self
    }
}

fn parse_rule(rule_str: &str) -> Rule {
    let mut rule_split = rule_str.split(": ");
    let r = Rule {
        field: String::from(rule_split.next().unwrap()),
        ranges: rule_split
            .next()
            .unwrap()
            .split(" or ")
            .map(|r| {
                let mut range_iter = r.split("-");
                (
                    range_iter.next().unwrap().parse::<u32>().unwrap(),
                    range_iter.next().unwrap().parse::<u32>().unwrap(),
                )
            })
            .collect::<Vec<(u32, u32)>>(),
    };
    r
}

#[aoc_generator(day16)]
pub fn input_generator(input: &str) -> Notes {
    let mut note_iter = input.split("\n\n");

    Notes {
        rules: note_iter
            .next()
            .unwrap()
            .lines()
            .map(|r| parse_rule(r))
            .collect::<Vec<Rule>>(),
        my_ticket: note_iter
            .next()
            .unwrap()
            .lines()
            .skip(1)
            .next()
            .unwrap()
            .split(",")
            .map(|v| v.parse::<u32>().unwrap())
            .collect::<Vec<u32>>(),
        nearby_tickets: note_iter
            .next()
            .unwrap()
            .lines()
            .skip(1)
            .map(|t| {
                t.split(",")
                    .map(|v| v.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>()
            })
            .collect::<Vec<Vec<u32>>>(),
    }
}
fn in_rule(rule: &Rule, value: &u32) -> bool {
    rule.ranges.iter().fold(false, |valid_inner, range| {
        valid_inner | (value >= &range.0 && value <= &range.1)
    })
}

fn in_rules(rules: &Vec<Rule>, value: &u32) -> bool {
    rules
        .iter()
        .fold(false, |valid, rule| valid | in_rule(rule, value))
}

fn all_in_rules(rules: &Vec<Rule>, values: &Vec<u32>) -> bool {
    values
        .iter()
        .fold(true, |valid, value| valid & in_rules(rules, value))
}

#[aoc(day16, part1)]
pub fn part1(notes: &Notes) -> u32 {
    notes.nearby_tickets.iter().fold(0u32, |error_rate, t| {
        t.iter().fold(0u32, |single_er, v| {
            if in_rules(&notes.rules, v) {
                single_er
            } else {
                single_er + v
            }
        }) + error_rate
    })
}

#[aoc(day16, part2)]
pub fn part2(notes: &Notes) -> u64 {
    let mut valid_nearby = Vec::<&Vec<u32>>::new();
    for nearby in &notes.nearby_tickets {
        if all_in_rules(&notes.rules, &nearby) {
            valid_nearby.push(nearby);
        }
    }

    // instantiate a vector for each feild with all possible rules
    let mut valid_rules = vec![notes.rules.clone(); notes.my_ticket.len()];
    // reduce down
    for nearby in valid_nearby {
        let mut new_valid_rules = Vec::<Vec<Rule>>::new();
        for (v_rules, value) in valid_rules.iter().zip(nearby) {
            let mut new_valid_rules_this = Vec::<Rule>::new();
            for rule in v_rules {
                if in_rule(rule, value) {
                    new_valid_rules_this.push(rule.clone());
                }
            }
            new_valid_rules.push(new_valid_rules_this);
        }
        valid_rules = new_valid_rules;
    }

    // we have all valid rules for each field. some have multiple options that can be
    // disambiguated
    // first convert to HashMap. TODO: alwys be a hashmap
    let hashed_valid_rules = valid_rules
        .iter()
        .map(|rules| {
            let mut mapp = HashMap::<String, Vec<(u32, u32)>>::new();
            for rule in rules {
                mapp.insert(rule.field.clone(), rule.ranges.clone());
            }
            mapp
        })
        .collect::<Vec<HashMap<String, Vec<(u32, u32)>>>>();

    // now disambiguate
    let mut disamb_rules = hashed_valid_rules.clone();
    loop {
        let mut disamb_rules_inner = disamb_rules.clone();
        for rules_map in disamb_rules.iter() {
            if rules_map.len() == 1 {
                let only_field = rules_map.keys().next().unwrap();
                // remove all other instances
                for d_rules in &mut disamb_rules_inner {
                    if d_rules.len() != 1 {
                        d_rules.remove(only_field);
                    }
                }
            }
        }
        disamb_rules = disamb_rules_inner.clone();

        let mut any_amb = false;
        for d_rules in &disamb_rules {
            if d_rules.len() != 1 {
                any_amb = true;
                break;
            }
        }
        if !any_amb {
            break;
        }
    }

    // get all those that have departure
    let mut prod: u64 = 1;
    for (index, rules) in disamb_rules.iter().enumerate() {
        let key = rules.keys().next().unwrap();
        if key.contains("departure") {
            prod *= notes.my_ticket[index] as u64;
        }
    }

    prod
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let sample = "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";
        assert_eq!(part1(&input_generator(sample)), 71);
    }

    #[test]
    fn trim() {
        let sample = "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";
        let notes = input_generator(sample);
        let mut valid_nearby = Vec::<&Vec<u32>>::new();
        for nearby in &notes.nearby_tickets {
            if all_in_rules(&notes.rules, &nearby) {
                valid_nearby.push(nearby);
            }
        }
        assert_eq!(valid_nearby.len(), 1);
    }

    #[test]
    fn sample2() {
        let sample = "class: 0-1 or 4-19
departure row: 0-5 or 8-19
departure seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9";
        assert_eq!(part2(&input_generator(sample)), 143);
    }
}
