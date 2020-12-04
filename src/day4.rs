use rayon::prelude::*;
use regex::Regex;

#[derive(Debug)]
pub struct Passport {
    ecl: Option<String>,
    pid: Option<String>,
    eyr: Option<u32>,
    hcl: Option<String>,
    byr: Option<u32>,
    cid: Option<u32>,
    hgt: Option<String>,
    iyr: Option<u32>,
}

impl Default for Passport {
    fn default() -> Passport {
        Passport {
            ecl: None,
            pid: None,
            eyr: None,
            hcl: None,
            byr: None,
            cid: None,
            hgt: None,
            iyr: None,
        }
    }
}

// #[aoc_generator(day4)]
// pub fn input_generator(input: &str) -> Vec<Passport> {
//     input
//         .split("\n\n")
//         .map(|l| {
//             let mut passport = Passport {
//                 ..Default::default()
//             };
//             for item in l.trim().split(&[' ', '\n'][..]) {
//                 let kv: Vec<&str> = item.trim().split(":").collect();
//                 match kv[0] {
//                     "ecl" => passport.ecl = Some(String::from(kv[1].trim())),
//                     "pid" => passport.pid = Some(String::from(kv[1].trim())),
//                     "eyr" => passport.eyr = Some(kv[1].trim().parse::<u32>().unwrap()),
//                     "hcl" => passport.hcl = Some(String::from(kv[1].trim())),
//                     "byr" => passport.byr = Some(kv[1].trim().parse::<u32>().unwrap()),
//                     "cid" => passport.cid = Some(kv[1].trim().parse::<u32>().unwrap()),
//                     "hgt" => passport.hgt = Some(String::from(kv[1].trim())),
//                     "iyr" => passport.iyr = Some(kv[1].trim().parse::<u32>().unwrap()),
//                     _ => {}
//                 }
//             }
//             passport
//         })
//         .collect()
// }

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<Passport> {
    let raw_passport = input.split("\n\n").collect::<Vec<&str>>();
    raw_passport
        .par_iter()
        .map(|l| {
            let mut passport = Passport {
                ..Default::default()
            };
            for item in l.trim().split(&[' ', '\n'][..]) {
                let kv: Vec<&str> = item.trim().split(":").map(|i| i.trim()).collect();
                if let Some(value) = kv.get(1) {
                    let s = String::from(value.trim());
                    match kv[0] {
                        "ecl" => passport.ecl = Some(s),
                        "pid" => passport.pid = Some(s),
                        "hcl" => passport.hcl = Some(s),
                        "hgt" => passport.hgt = Some(s),
                        _ => {}
                    }
                    if let Ok(n) = value.trim().parse::<u32>() {
                        match kv[0] {
                            "byr" => passport.byr = Some(n),
                            "cid" => passport.cid = Some(n),
                            "eyr" => passport.eyr = Some(n),
                            "iyr" => passport.iyr = Some(n),
                            _ => {}
                        }
                    }
                }
            }
            passport
        })
        .collect()
}

#[aoc(day4, part1, parallel)]
pub fn part1_parallel(passports: &Vec<Passport>) -> u32 {
    passports
        .par_iter()
        .map(|p| {
            (p.ecl.is_some()
                && p.pid.is_some()
                && p.eyr.is_some()
                && p.hcl.is_some()
                && p.byr.is_some()
                && p.hgt.is_some()
                && p.iyr.is_some()) as u32
        })
        .sum::<u32>()
}

#[aoc(day4, part2, parallel)]
pub fn part2_parallel(passports: &Vec<Passport>) -> u32 {
    // hcl validity
    let hcl_re = Regex::new("^#(?:[0-9a-f]{6})$").unwrap();

    // ecl validity
    let ecl_re = Regex::new("(amb)|(blu)|(brn)|(gry)|(grn)|(hzl)|(oth)").unwrap();

    passports
        .par_iter()
        .map(|passport| {
            // hgt validity
            let mut hgt_valid = false;
            if let Some(hgt_string) = &passport.hgt {
                if hgt_string.contains("in") {
                    let hgt_string_numeric = hgt_string.replace("in", "");
                    let hgt_numeric = hgt_string_numeric.parse::<u32>().unwrap();
                    hgt_valid = (hgt_numeric >= 59) && (hgt_numeric <= 76);
                } else if hgt_string.contains("cm") {
                    let hgt_string_numeric = hgt_string.replace("cm", "");
                    let hgt_numeric = hgt_string_numeric.parse::<u32>().unwrap();
                    hgt_valid = (hgt_numeric >= 150) && (hgt_numeric <= 193);
                }
            }

            (passport.ecl.is_some()
                && passport.pid.is_some()
                && passport.eyr.is_some()
                && passport.hcl.is_some()
                && passport.byr.is_some()
                && passport.hgt.is_some()
                && passport.iyr.is_some()
                && passport.byr.unwrap() >= 1920
                && passport.byr.unwrap() <= 2002
                && passport.iyr.unwrap() >= 2010
                && passport.iyr.unwrap() <= 2020
                && passport.eyr.unwrap() >= 2020
                && passport.eyr.unwrap() <= 2030
                && hgt_valid
                && hcl_re.is_match(passport.hcl.as_ref().unwrap())
                && ecl_re.is_match(passport.ecl.as_ref().unwrap())
                && passport.pid.as_ref().unwrap().chars().count() == 9) as u32
        })
        .sum::<u32>()
}

#[aoc(day4, part1)]
pub fn part1(passports: &Vec<Passport>) -> u32 {
    let mut count = 0;
    for passport in passports {
        if passport.ecl.is_some()
            && passport.pid.is_some()
            && passport.eyr.is_some()
            && passport.hcl.is_some()
            && passport.byr.is_some()
            && passport.hgt.is_some()
            && passport.iyr.is_some()
        {
            count += 1;
        }
    }
    count
}

#[aoc(day4, part2)]
pub fn part2(passports: &Vec<Passport>) -> u32 {
    let mut count = 0;

    // hcl validity
    let hcl_re = Regex::new("^#(?:[0-9a-f]{6})$").unwrap();

    // ecl validity
    let ecl_re = Regex::new("(amb)|(blu)|(brn)|(gry)|(grn)|(hzl)|(oth)").unwrap();

    for passport in passports {
        // hgt validity
        let mut hgt_valid = false;
        if let Some(hgt_string) = &passport.hgt {
            if hgt_string.contains("in") {
                let hgt_string_numeric = hgt_string.replace("in", "");
                let hgt_numeric = hgt_string_numeric.parse::<u32>().unwrap();
                hgt_valid = (hgt_numeric >= 59) && (hgt_numeric <= 76);
            } else if hgt_string.contains("cm") {
                let hgt_string_numeric = hgt_string.replace("cm", "");
                let hgt_numeric = hgt_string_numeric.parse::<u32>().unwrap();
                hgt_valid = (hgt_numeric >= 150) && (hgt_numeric <= 193);
            }
        }

        if passport.ecl.is_some()
            && passport.pid.is_some()
            && passport.eyr.is_some()
            && passport.hcl.is_some()
            && passport.byr.is_some()
            && passport.hgt.is_some()
            && passport.iyr.is_some()
            && passport.byr.unwrap() >= 1920
            && passport.byr.unwrap() <= 2002
            && passport.iyr.unwrap() >= 2010
            && passport.iyr.unwrap() <= 2020
            && passport.eyr.unwrap() >= 2020
            && passport.eyr.unwrap() <= 2030
            && hgt_valid
            && hcl_re.is_match(passport.hcl.as_ref().unwrap())
            && ecl_re.is_match(passport.ecl.as_ref().unwrap())
            && passport.pid.as_ref().unwrap().chars().count() == 9
        {
            count += 1;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let sample = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
                        byr:1937 iyr:2017 cid:147 hgt:183cm

                        iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
                        hcl:#cfa07d byr:1929

                        hcl:#ae17e1 iyr:2013
                        eyr:2024
                        ecl:brn pid:760753108 byr:1931
                        hgt:179cm
                        
                        hcl:#cfa07d eyr:2025 pid:166559648
                        iyr:2011 ecl:brn hgt:59in";
        assert_eq!(part1(&input_generator(sample)), 2);
    }

    #[test]
    fn sample2() {
        let sample = "eyr:1972 cid:100
                        hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

                        iyr:2019
                        hcl:#602927 eyr:1967 hgt:170cm
                        ecl:grn pid:012533040 byr:1946

                        hcl:dab227 iyr:2012
                        ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

                        hgt:59cm ecl:zzz
                        eyr:2038 hcl:74454a iyr:2023
                        pid:3556412378 byr:2007";
        assert_eq!(part2(&input_generator(sample)), 0);
    }

    #[test]
    fn sample3() {
        let sample = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
                        hcl:#623a2f

                        eyr:2029 ecl:blu cid:129 byr:1989
                        iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

                        hcl:#888785
                        hgt:164cm byr:2001 iyr:2015 cid:88
                        pid:545766238 ecl:hzl
                        eyr:2022

                        iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";
        assert_eq!(part2(&input_generator(sample)), 4);
    }
}
