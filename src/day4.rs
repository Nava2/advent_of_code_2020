use std::hash::Hash;
use std::marker::Copy;
use std::cmp::PartialEq;
use std::num::ParseIntError;
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub enum PassportField {
    BirthYear(u32), // byr
    IssueYear(u32), // iyr
    ExpirationYear(u32), // eyr
    Height(u32), // hgt
    HairColor(String), // hcl
    EyeColor(String), // ecl
    PassportId(u32), // pid
    CountryId(u32), // cid
}

fn parse_u32(key: &str, value: &str) -> Result<u32, String> {
    value.parse::<u32>()
        .map_err(|_| format!("Failed to parse u32 key={:?} value={:?}", key, value))
}

impl PassportField {
    pub fn parse(str_pair: &str) -> Result<PassportField, String> {
        let pair = str_pair.split(PAIR_SEPARATOR).collect::<Vec<&str>>();
        let key = pair[0];
        let value = pair[1];

        match key {
            "byr" => Ok(PassportField::BirthYear(parse_u32(key, value)?)),
            "iyr" => Ok(PassportField::IssueYear(parse_u32(key, value)?)),
            "eyr" => Ok(PassportField::ExpirationYear(parse_u32(key, value)?)),
            "hgt" => {
                let value_len = value.chars().count();
                let cm_tag = value.chars().skip(value_len - 2).collect::<String>();
                if cm_tag != "cm" {
                    return Err(format!("Incorrectly formatted key={:?} value={:?}", key, value))
                }

                let chars = value.chars().take(value_len - 2).collect::<String>();
                let height = parse_u32(key, &chars)?;
                Ok(PassportField::Height(height))
            },
            "hcl" => Ok(PassportField::HairColor(String::from(value))),
            "ecl" => Ok(PassportField::EyeColor(String::from(value))),
            "pid" => Ok(PassportField::PassportId(parse_u32(key, value)?)),
            "cid" => Ok(PassportField::CountryId(parse_u32(key, value)?)),
            _ => Err(format!("Unknown key={:?}", key))
        }
    }
}

const PAIR_SEPARATOR: char = ':';

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<HashSet<PassportField>> {
    let mut result = Vec::new();
    let mut current_fields = HashSet::<PassportField>::new();
    for line in input.lines() {
        if line.chars().count() == 0 {
            result.push(current_fields);
            current_fields = HashSet::new();
        }

        for field in line.split_whitespace().map(|pair| PassportField::parse(pair)) {
            match field {
                Ok(f) => {
                    current_fields.insert(f);
                },
                Err(m) => { println!("{}", m) },
            };
        }
    }

    result
}

#[aoc(day4, part1)]
pub fn solve_part1(passports: &[HashSet<PassportField>]) -> usize {
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn passport_fields_height() {
        assert_eq!(PassportField::parse("hgt:20"), Err(String::from("Incorrectly formatted value=\"20\"")));
        assert_eq!(PassportField::parse("hgt:aa"), Err(String::from("Incorrectly formatted value=\"aa\"")));
        assert_eq!(PassportField::parse("hgt:aacm"), Err(String::from("Failed to parse u32 key=\"hgt\" value=\"aa\"")));

        assert_eq!(PassportField::parse("hgt:20cm"), Ok(PassportField::Height(20)));
    }
}