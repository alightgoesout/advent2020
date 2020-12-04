use im_rc::{HashMap, Vector};

use super::input::read_lines;

const KEY_VALUE_SEPARATOR: char = ':';

pub fn execute() {
    let passports = parse_passports(read_lines("day4").unwrap());
    println!(
        "4:1 — Number of valid passports: {}",
        count_valid_passports(&passports)
    );
}

fn count_valid_passports(passports: &Vector<Passport>) -> usize {
    passports
        .iter()
        .map(is_valid)
        .filter(|valid| *valid)
        .count()
}

type Passport = HashMap<String, String>;

fn is_valid(passport: &Passport) -> bool {
    match passport.len() {
        8 => true,
        7 => passport.keys().all(|key| key.as_str() != "cid"),
        _ => false,
    }
}

fn parse_passports(lines: Vector<String>) -> Vector<Passport> {
    group_by_passport(lines)
        .into_iter()
        .map(parse_passport)
        .collect()
}

fn parse_passport(passport_lines: Vector<String>) -> Passport {
    passport_lines
        .iter()
        .map(|line| {
            line.split(' ').map(|field| {
                let entry = field.splitn(2, KEY_VALUE_SEPARATOR).collect::<Vector<_>>();
                (entry[0].into(), entry[1].into())
            })
        })
        .flatten()
        .collect()
}

fn group_by_passport(lines: Vector<String>) -> Vector<Vector<String>> {
    let mut passports = Vector::new();
    let mut accumulator = Vector::new();
    for line in lines {
        if line.is_empty() {
            passports.push_back(accumulator);
            accumulator = Vector::new();
        } else {
            accumulator.push_back(line)
        }
    }
    if !accumulator.is_empty() {
        passports.push_back(accumulator);
    }
    passports
}

#[cfg(test)]
mod group_by_passport_should {
    use im_rc::vector;

    use super::*;

    #[test]
    fn return_a_vector_of_size_one_when_there_is_only_one_line() {
        let lines: Vector<String> = vector!["ecl:gry pid:860033327 eyr:2020 hcl:#fffffd".into()];

        let result = group_by_passport(lines);

        assert_eq!(
            result,
            vector![vector!["ecl:gry pid:860033327 eyr:2020 hcl:#fffffd".into()]]
        );
    }

    #[test]
    fn return_a_vector_of_size_two_when_there_is_only_two_one_line_passports() {
        let lines: Vector<String> = vector![
            "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd".into(),
            "".into(),
            "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd".into(),
        ];

        let result = group_by_passport(lines);

        assert_eq!(
            result,
            vector![
                vector!["ecl:gry pid:860033327 eyr:2020 hcl:#fffffd".into()],
                vector!["ecl:gry pid:860033327 eyr:2020 hcl:#fffffd".into()],
            ]
        );
    }

    #[test]
    fn return_a_vector_of_size_one_when_there_is_one_passport_on_two_lines() {
        let lines: Vector<String> = vector![
            "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd".into(),
            "byr:1937 iyr:2017 cid:147 hgt:183cm".into(),
        ];

        let result = group_by_passport(lines);

        assert_eq!(
            result,
            vector![vector![
                "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd".into(),
                "byr:1937 iyr:2017 cid:147 hgt:183cm".into(),
            ]]
        );
    }

    #[test]
    fn ignore_a_last_empty_line() {
        let lines: Vector<String> = vector![
            "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd".into(),
            "byr:1937 iyr:2017 cid:147 hgt:183cm".into(),
            "".into(),
        ];

        let result = group_by_passport(lines);

        assert_eq!(
            result,
            vector![vector![
                "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd".into(),
                "byr:1937 iyr:2017 cid:147 hgt:183cm".into(),
            ]]
        );
    }

    #[test]
    fn return_a_vector_of_size_two_when_there_is_only_two_multiline_passports() {
        let lines: Vector<String> = vector![
            "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd".into(),
            "byr:1937 iyr:2017 cid:147 hgt:183cm".into(),
            "".into(),
            "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd".into(),
        ];

        let result = group_by_passport(lines);

        assert_eq!(
            result,
            vector![
                vector![
                    "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd".into(),
                    "byr:1937 iyr:2017 cid:147 hgt:183cm".into(),
                ],
                vector!["ecl:gry pid:860033327 eyr:2020 hcl:#fffffd".into()],
            ]
        );
    }
}

#[cfg(test)]
mod parse_passport_should {
    use im_rc::{hashmap, vector};

    use super::*;

    #[test]
    fn return_a_passport_with_one_field_when_there_is_one_line_with_one_field() {
        let passport_lines = vector!["ecl:gry".into()];

        let result = parse_passport(passport_lines);

        assert_eq!(
            result,
            hashmap! {
                "ecl".into() => "gry".into(),
            }
        )
    }

    #[test]
    fn return_a_passport_with_two_fields_when_there_is_one_line_with_two_fields() {
        let passport_lines = vector!["ecl:gry pid:860033327".into()];

        let result = parse_passport(passport_lines);

        assert_eq!(
            result,
            hashmap! {
                "ecl".into() => "gry".into(),
                "pid".into() => "860033327".into(),
            }
        )
    }

    #[test]
    fn return_a_passport_with_two_fields_when_there_is_two_lines_with_one_field_each() {
        let passport_lines = vector!["ecl:gry".into(), "pid:860033327".into()];

        let result = parse_passport(passport_lines);

        assert_eq!(
            result,
            hashmap! {
                "ecl".into() => "gry".into(),
                "pid".into() => "860033327".into(),
            }
        )
    }

    #[test]
    fn return_a_passport_with_8_fields_when_there_is_two_lines_with_four_field_each() {
        let passport_lines = vector![
            "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd".into(),
            "byr:1937 iyr:2017 cid:147 hgt:183cm".into(),
        ];

        let result = parse_passport(passport_lines);

        assert_eq!(
            result,
            hashmap! {
                "ecl".into() => "gry".into(),
                "pid".into() => "860033327".into(),
                "eyr".into() => "2020".into(),
                "hcl".into() => "#fffffd".into(),
                "byr".into() => "1937".into(),
                "iyr".into() => "2017".into(),
                "cid".into() => "147".into(),
                "hgt".into() => "183cm".into(),
            }
        )
    }
}

#[cfg(test)]
mod is_valid_should {
    use im_rc::hashmap;

    use super::*;

    #[test]
    fn return_false_when_there_is_one_field() {
        let passport = hashmap! {"ecl".into() => "gry".into()};

        assert!(!is_valid(&passport));
    }

    #[test]
    fn return_false_when_there_are_two_fields() {
        let passport = hashmap! {
            "ecl".into() => "gry".into(),
            "pid".into() => "860033327".into(),
        };

        assert!(!is_valid(&passport));
    }

    #[test]
    fn return_false_when_there_are_three_fields() {
        let passport = hashmap! {
            "ecl".into() => "gry".into(),
            "pid".into() => "860033327".into(),
            "eyr".into() => "2020".into(),
        };

        assert!(!is_valid(&passport));
    }

    #[test]
    fn return_false_when_there_are_four_fields() {
        let passport = hashmap! {
            "ecl".into() => "gry".into(),
            "pid".into() => "860033327".into(),
            "eyr".into() => "2020".into(),
            "hcl".into() => "#fffffd".into(),
        };

        assert!(!is_valid(&passport));
    }

    #[test]
    fn return_false_when_there_are_five_fields() {
        let passport = hashmap! {
            "ecl".into() => "gry".into(),
            "pid".into() => "860033327".into(),
            "eyr".into() => "2020".into(),
            "hcl".into() => "#fffffd".into(),
            "byr".into() => "1937".into(),
        };

        assert!(!is_valid(&passport));
    }

    #[test]
    fn return_false_when_there_are_six_fields() {
        let passport = hashmap! {
            "ecl".into() => "gry".into(),
            "pid".into() => "860033327".into(),
            "eyr".into() => "2020".into(),
            "hcl".into() => "#fffffd".into(),
            "byr".into() => "1937".into(),
            "iyr".into() => "2017".into(),
        };

        assert!(!is_valid(&passport));
    }

    #[test]
    fn return_true_when_there_are_eight_fields() {
        let passport = hashmap! {
            "ecl".into() => "gry".into(),
            "pid".into() => "860033327".into(),
            "eyr".into() => "2020".into(),
            "hcl".into() => "#fffffd".into(),
            "byr".into() => "1937".into(),
            "iyr".into() => "2017".into(),
            "cid".into() => "147".into(),
            "hgt".into() => "183cm".into(),
        };

        assert!(is_valid(&passport));
    }

    #[test]
    fn return_false_when_ecl_is_missing() {
        let passport = hashmap! {
            "pid".into() => "860033327".into(),
            "eyr".into() => "2020".into(),
            "hcl".into() => "#fffffd".into(),
            "byr".into() => "1937".into(),
            "iyr".into() => "2017".into(),
            "cid".into() => "147".into(),
            "hgt".into() => "183cm".into(),
        };

        assert!(!is_valid(&passport));
    }

    #[test]
    fn return_false_when_pid_is_missing() {
        let passport = hashmap! {
            "ecl".into() => "gry".into(),
            "eyr".into() => "2020".into(),
            "hcl".into() => "#fffffd".into(),
            "byr".into() => "1937".into(),
            "iyr".into() => "2017".into(),
            "cid".into() => "147".into(),
            "hgt".into() => "183cm".into(),
        };

        assert!(!is_valid(&passport));
    }

    #[test]
    fn return_false_when_eyr_is_missing() {
        let passport = hashmap! {
            "ecl".into() => "gry".into(),
            "pid".into() => "860033327".into(),
            "hcl".into() => "#fffffd".into(),
            "byr".into() => "1937".into(),
            "iyr".into() => "2017".into(),
            "cid".into() => "147".into(),
            "hgt".into() => "183cm".into(),
        };

        assert!(!is_valid(&passport));
    }

    #[test]
    fn return_false_when_hcl_is_missing() {
        let passport = hashmap! {
            "ecl".into() => "gry".into(),
            "pid".into() => "860033327".into(),
            "eyr".into() => "2020".into(),
            "byr".into() => "1937".into(),
            "iyr".into() => "2017".into(),
            "cid".into() => "147".into(),
            "hgt".into() => "183cm".into(),
        };

        assert!(!is_valid(&passport));
    }

    #[test]
    fn return_false_when_byr_is_missing() {
        let passport = hashmap! {
            "ecl".into() => "gry".into(),
            "pid".into() => "860033327".into(),
            "eyr".into() => "2020".into(),
            "hcl".into() => "#fffffd".into(),
            "iyr".into() => "2017".into(),
            "cid".into() => "147".into(),
            "hgt".into() => "183cm".into(),
        };

        assert!(!is_valid(&passport));
    }

    #[test]
    fn return_false_when_iyr_is_missing() {
        let passport = hashmap! {
            "ecl".into() => "gry".into(),
            "pid".into() => "860033327".into(),
            "eyr".into() => "2020".into(),
            "hcl".into() => "#fffffd".into(),
            "byr".into() => "1937".into(),
            "cid".into() => "147".into(),
            "hgt".into() => "183cm".into(),
        };

        assert!(!is_valid(&passport));
    }

    #[test]
    fn return_false_when_hgt_is_missing() {
        let passport = hashmap! {
            "ecl".into() => "gry".into(),
            "pid".into() => "860033327".into(),
            "eyr".into() => "2020".into(),
            "hcl".into() => "#fffffd".into(),
            "byr".into() => "1937".into(),
            "iyr".into() => "2017".into(),
            "cid".into() => "147".into(),
        };

        assert!(!is_valid(&passport));
    }

    #[test]
    fn return_true_when_cid_is_missing() {
        let passport = hashmap! {
            "ecl".into() => "gry".into(),
            "pid".into() => "860033327".into(),
            "eyr".into() => "2020".into(),
            "hcl".into() => "#fffffd".into(),
            "byr".into() => "1937".into(),
            "iyr".into() => "2017".into(),
            "hgt".into() => "183cm".into(),
        };

        assert!(is_valid(&passport));
    }
}
