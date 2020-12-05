use lazy_static::lazy_static;
use regex::Regex;

use super::input::read_lines;
use std::collections::HashMap;
use std::ops::RangeBounds;

const KEY_VALUE_SEPARATOR: char = ':';

pub fn execute() {
    let passports = parse_passports(read_lines("day4").unwrap());
    println!(
        "4:1 — Number of valid passports: {}",
        count_valid_passports(&passports)
    );
    println!(
        "4:2 — Number of fully valid passports: {}",
        count_fully_valid_passports(&passports)
    );
}

fn count_valid_passports(passports: &[Passport]) -> usize {
    passports
        .iter()
        .map(is_valid)
        .filter(|valid| *valid)
        .count()
}

fn count_fully_valid_passports(passports: &[Passport]) -> usize {
    passports
        .iter()
        .map(is_fully_valid)
        .filter(|valid| *valid)
        .count()
}

type Passport = HashMap<String, String>;

fn is_fully_valid(passport: &Passport) -> bool {
    is_valid(passport)
        && passport
            .iter()
            .all(|(key, value)| is_valid_field(key, value))
}

fn is_valid_field(key: &str, value: &str) -> bool {
    match key {
        "byr" => is_number_within_range(value, 1920..=2002),
        "iyr" => is_number_within_range(value, 2010..=2020),
        "eyr" => is_number_within_range(value, 2020..=2030),
        "hgt" => is_valid_height(value),
        "hcl" => is_valid_hair_color(value),
        "ecl" => is_valid_eye_color(value),
        "pid" => is_valid_pid(value),
        "cid" => true,
        _ => false,
    }
}

fn is_number_within_range(value: &str, range: impl RangeBounds<u32>) -> bool {
    value
        .parse::<u32>()
        .ok()
        .map(|year| range.contains(&year))
        .unwrap_or(false)
}

lazy_static! {
    static ref HEIGHT_REGEX: Regex = Regex::new(r"^(?P<value>\d+)(?P<unit>\w{2})$").unwrap();
}

fn is_valid_height(height: &str) -> bool {
    match HEIGHT_REGEX.captures(height) {
        Some(captures) => {
            let value = captures.name("value").unwrap().as_str();
            let unit = captures.name("unit").unwrap().as_str();
            match unit {
                "cm" => is_number_within_range(value, 150..=193),
                "in" => is_number_within_range(value, 59..=76),
                _ => false,
            }
        }
        None => false,
    }
}

lazy_static! {
    static ref HAIR_COLOR_REGEX: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
}

fn is_valid_hair_color(hair_color: &str) -> bool {
    HAIR_COLOR_REGEX.is_match(hair_color)
}

const VALID_EYE_COLORS: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

fn is_valid_eye_color(eye_color: &str) -> bool {
    VALID_EYE_COLORS.contains(&eye_color)
}

lazy_static! {
    static ref PID_REGEX: Regex = Regex::new(r"^\d{9}$").unwrap();
}

fn is_valid_pid(pid: &str) -> bool {
    PID_REGEX.is_match(pid)
}

fn is_valid(passport: &Passport) -> bool {
    match passport.len() {
        8 => true,
        7 => passport.keys().all(|key| key != "cid"),
        _ => false,
    }
}

fn parse_passports(lines: Vec<String>) -> Vec<Passport> {
    group_by_passport(lines)
        .into_iter()
        .map(parse_passport)
        .collect()
}

fn parse_passport(passport_lines: Vec<String>) -> Passport {
    passport_lines
        .iter()
        .map(|line| {
            line.split(' ').map(|field| {
                let entry = field.splitn(2, KEY_VALUE_SEPARATOR).collect::<Vec<_>>();
                (entry[0].into(), entry[1].into())
            })
        })
        .flatten()
        .collect()
}

fn group_by_passport(lines: Vec<String>) -> Vec<Vec<String>> {
    let mut passports = Vec::new();
    let mut accumulator = Vec::new();
    for line in lines {
        if line.is_empty() {
            passports.push(accumulator);
            accumulator = Vec::new();
        } else {
            accumulator.push(line)
        }
    }
    if !accumulator.is_empty() {
        passports.push(accumulator);
    }
    passports
}

#[cfg(test)]
mod group_by_passport_should {
    use super::*;

    #[test]
    fn return_a_vec_of_size_one_when_there_is_only_one_line() {
        let lines: Vec<String> = vec!["ecl:gry pid:860033327 eyr:2020 hcl:#fffffd".into()];

        let result = group_by_passport(lines);

        assert_eq!(
            result,
            vec![vec![
                "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd".to_string()
            ]]
        );
    }

    #[test]
    fn return_a_vec_of_size_two_when_there_is_only_two_one_line_passports() {
        let lines: Vec<String> = vec![
            "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd".into(),
            "".into(),
            "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd".into(),
        ];

        let result = group_by_passport(lines);

        assert_eq!(
            result,
            vec![
                vec!["ecl:gry pid:860033327 eyr:2020 hcl:#fffffd".to_string()],
                vec!["ecl:gry pid:860033327 eyr:2020 hcl:#fffffd".to_string()],
            ]
        );
    }

    #[test]
    fn return_a_vec_of_size_one_when_there_is_one_passport_on_two_lines() {
        let lines: Vec<String> = vec![
            "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd".into(),
            "byr:1937 iyr:2017 cid:147 hgt:183cm".into(),
        ];

        let result = group_by_passport(lines);

        assert_eq!(
            result,
            vec![vec![
                "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd".to_string(),
                "byr:1937 iyr:2017 cid:147 hgt:183cm".to_string(),
            ]]
        );
    }

    #[test]
    fn ignore_a_last_empty_line() {
        let lines: Vec<String> = vec![
            "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd".into(),
            "byr:1937 iyr:2017 cid:147 hgt:183cm".into(),
            "".into(),
        ];

        let result = group_by_passport(lines);

        assert_eq!(
            result,
            vec![vec![
                "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd".to_string(),
                "byr:1937 iyr:2017 cid:147 hgt:183cm".to_string(),
            ]]
        );
    }

    #[test]
    fn return_a_vec_of_size_two_when_there_is_only_two_multiline_passports() {
        let lines: Vec<String> = vec![
            "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd".into(),
            "byr:1937 iyr:2017 cid:147 hgt:183cm".into(),
            "".into(),
            "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd".into(),
        ];

        let result = group_by_passport(lines);

        assert_eq!(
            result,
            vec![
                vec![
                    "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd".to_string(),
                    "byr:1937 iyr:2017 cid:147 hgt:183cm".to_string(),
                ],
                vec!["ecl:gry pid:860033327 eyr:2020 hcl:#fffffd".into()],
            ]
        );
    }
}

#[cfg(test)]
mod parse_passport_should {
    use super::*;

    #[test]
    fn return_a_passport_with_one_field_when_there_is_one_line_with_one_field() {
        let passport_lines = vec!["ecl:gry".into()];

        let result = parse_passport(passport_lines);

        assert_eq!(
            result,
            vec![("ecl".into(), "gry".into()),].into_iter().collect(),
        )
    }

    #[test]
    fn return_a_passport_with_two_fields_when_there_is_one_line_with_two_fields() {
        let passport_lines = vec!["ecl:gry pid:860033327".into()];

        let result = parse_passport(passport_lines);

        assert_eq!(
            result,
            vec![
                ("ecl".into(), "gry".into()),
                ("pid".into(), "860033327".into()),
            ]
            .into_iter()
            .collect(),
        )
    }

    #[test]
    fn return_a_passport_with_two_fields_when_there_is_two_lines_with_one_field_each() {
        let passport_lines = vec!["ecl:gry".into(), "pid:860033327".into()];

        let result = parse_passport(passport_lines);

        assert_eq!(
            result,
            vec![
                ("ecl".into(), "gry".into()),
                ("pid".into(), "860033327".into()),
            ]
            .into_iter()
            .collect(),
        )
    }

    #[test]
    fn return_a_passport_with_8_fields_when_there_is_two_lines_with_four_field_each() {
        let passport_lines = vec![
            "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd".into(),
            "byr:1937 iyr:2017 cid:147 hgt:183cm".into(),
        ];

        let result = parse_passport(passport_lines);

        assert_eq!(
            result,
            vec![
                ("ecl".into(), "gry".into()),
                ("pid".into(), "860033327".into()),
                ("eyr".into(), "2020".into()),
                ("hcl".into(), "#fffffd".into()),
                ("byr".into(), "1937".into()),
                ("iyr".into(), "2017".into()),
                ("cid".into(), "147".into()),
                ("hgt".into(), "183cm".into()),
            ]
            .into_iter()
            .collect(),
        )
    }
}

#[cfg(test)]
mod is_valid_should {
    use super::*;

    #[test]
    fn return_false_when_there_is_one_field() {
        let passport = vec![("ecl".into(), "gry".into())].into_iter().collect();

        assert!(!is_valid(&passport));
    }

    #[test]
    fn return_false_when_there_are_two_fields() {
        let passport = vec![
            ("ecl".into(), "gry".into()),
            ("pid".into(), "860033327".into()),
        ]
        .into_iter()
        .collect();

        assert!(!is_valid(&passport));
    }

    #[test]
    fn return_false_when_there_are_three_fields() {
        let passport = vec![
            ("ecl".into(), "gry".into()),
            ("pid".into(), "860033327".into()),
            ("eyr".into(), "2020".into()),
        ]
        .into_iter()
        .collect();

        assert!(!is_valid(&passport));
    }

    #[test]
    fn return_false_when_there_are_four_fields() {
        let passport = vec![
            ("ecl".into(), "gry".into()),
            ("pid".into(), "860033327".into()),
            ("eyr".into(), "2020".into()),
            ("hcl".into(), "#fffffd".into()),
        ]
        .into_iter()
        .collect();

        assert!(!is_valid(&passport));
    }

    #[test]
    fn return_false_when_there_are_five_fields() {
        let passport = vec![
            ("ecl".into(), "gry".into()),
            ("pid".into(), "860033327".into()),
            ("eyr".into(), "2020".into()),
            ("hcl".into(), "#fffffd".into()),
            ("byr".into(), "1937".into()),
        ]
        .into_iter()
        .collect();

        assert!(!is_valid(&passport));
    }

    #[test]
    fn return_false_when_there_are_six_fields() {
        let passport = vec![
            ("ecl".into(), "gry".into()),
            ("pid".into(), "860033327".into()),
            ("eyr".into(), "2020".into()),
            ("hcl".into(), "#fffffd".into()),
            ("byr".into(), "1937".into()),
            ("iyr".into(), "2017".into()),
        ]
        .into_iter()
        .collect();

        assert!(!is_valid(&passport));
    }

    #[test]
    fn return_true_when_there_are_eight_fields() {
        let passport = vec![
            ("ecl".into(), "gry".into()),
            ("pid".into(), "860033327".into()),
            ("eyr".into(), "2020".into()),
            ("hcl".into(), "#fffffd".into()),
            ("byr".into(), "1937".into()),
            ("iyr".into(), "2017".into()),
            ("cid".into(), "147".into()),
            ("hgt".into(), "183cm".into()),
        ]
        .into_iter()
        .collect();

        assert!(is_valid(&passport));
    }

    #[test]
    fn return_false_when_ecl_is_missing() {
        let passport = vec![
            ("pid".into(), "860033327".into()),
            ("eyr".into(), "2020".into()),
            ("hcl".into(), "#fffffd".into()),
            ("byr".into(), "1937".into()),
            ("iyr".into(), "2017".into()),
            ("cid".into(), "147".into()),
            ("hgt".into(), "183cm".into()),
        ]
        .into_iter()
        .collect();

        assert!(!is_valid(&passport));
    }

    #[test]
    fn return_false_when_pid_is_missing() {
        let passport = vec![
            ("ecl".into(), "gry".into()),
            ("eyr".into(), "2020".into()),
            ("hcl".into(), "#fffffd".into()),
            ("byr".into(), "1937".into()),
            ("iyr".into(), "2017".into()),
            ("cid".into(), "147".into()),
            ("hgt".into(), "183cm".into()),
        ]
        .into_iter()
        .collect();

        assert!(!is_valid(&passport));
    }

    #[test]
    fn return_false_when_eyr_is_missing() {
        let passport = vec![
            ("ecl".into(), "gry".into()),
            ("pid".into(), "860033327".into()),
            ("hcl".into(), "#fffffd".into()),
            ("byr".into(), "1937".into()),
            ("iyr".into(), "2017".into()),
            ("cid".into(), "147".into()),
            ("hgt".into(), "183cm".into()),
        ]
        .into_iter()
        .collect();

        assert!(!is_valid(&passport));
    }

    #[test]
    fn return_false_when_hcl_is_missing() {
        let passport = vec![
            ("ecl".into(), "gry".into()),
            ("pid".into(), "860033327".into()),
            ("eyr".into(), "2020".into()),
            ("byr".into(), "1937".into()),
            ("iyr".into(), "2017".into()),
            ("cid".into(), "147".into()),
            ("hgt".into(), "183cm".into()),
        ]
        .into_iter()
        .collect();

        assert!(!is_valid(&passport));
    }

    #[test]
    fn return_false_when_byr_is_missing() {
        let passport = vec![
            ("ecl".into(), "gry".into()),
            ("pid".into(), "860033327".into()),
            ("eyr".into(), "2020".into()),
            ("hcl".into(), "#fffffd".into()),
            ("iyr".into(), "2017".into()),
            ("cid".into(), "147".into()),
            ("hgt".into(), "183cm".into()),
        ]
        .into_iter()
        .collect();

        assert!(!is_valid(&passport));
    }

    #[test]
    fn return_false_when_iyr_is_missing() {
        let passport = vec![
            ("ecl".into(), "gry".into()),
            ("pid".into(), "860033327".into()),
            ("eyr".into(), "2020".into()),
            ("hcl".into(), "#fffffd".into()),
            ("byr".into(), "1937".into()),
            ("cid".into(), "147".into()),
            ("hgt".into(), "183cm".into()),
        ]
        .into_iter()
        .collect();

        assert!(!is_valid(&passport));
    }

    #[test]
    fn return_false_when_hgt_is_missing() {
        let passport = vec![
            ("ecl".into(), "gry".into()),
            ("pid".into(), "860033327".into()),
            ("eyr".into(), "2020".into()),
            ("hcl".into(), "#fffffd".into()),
            ("byr".into(), "1937".into()),
            ("iyr".into(), "2017".into()),
            ("cid".into(), "147".into()),
        ]
        .into_iter()
        .collect();

        assert!(!is_valid(&passport));
    }

    #[test]
    fn return_true_when_cid_is_missing() {
        let passport = vec![
            ("ecl".into(), "gry".into()),
            ("pid".into(), "860033327".into()),
            ("eyr".into(), "2020".into()),
            ("hcl".into(), "#fffffd".into()),
            ("byr".into(), "1937".into()),
            ("iyr".into(), "2017".into()),
            ("hgt".into(), "183cm".into()),
        ]
        .into_iter()
        .collect();

        assert!(is_valid(&passport));
    }
}

#[cfg(test)]
mod is_fully_valid_should {
    use super::*;

    #[test]
    fn return_false_for_invalid_passwords() {
        let invalid_passports = parse_passports(vec![
            "eyr:1972 cid:100".into(),
            "hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926".into(),
            "".into(),
            "iyr:2019".into(),
            "hcl:#602927 eyr:1967 hgt:170cm".into(),
            "ecl:grn pid:012533040 byr:1946".into(),
            "".into(),
            "hcl:dab227 iyr:2012".into(),
            "ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277".into(),
            "".into(),
            "hgt:59cm ecl:zzz".into(),
            "eyr:2038 hcl:74454a iyr:2023".into(),
            "pid:3556412378 byr:2007".into(),
        ]);

        assert!(invalid_passports
            .iter()
            .all(|passport| !is_fully_valid(passport)))
    }

    #[test]
    fn return_true_for_valid_passwords() {
        let valid_passports = parse_passports(vec![
            "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980".into(),
            "hcl:#623a2f".into(),
            "".into(),
            "eyr:2029 ecl:blu cid:129 byr:1989".into(),
            "iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm".into(),
            "".into(),
            "hcl:#888785".into(),
            "hgt:164cm byr:2001 iyr:2015 cid:88".into(),
            "pid:545766238 ecl:hzl".into(),
            "eyr:2022".into(),
            "".into(),
            "iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719".into(),
        ]);

        assert!(valid_passports.iter().all(is_fully_valid));
    }
}
