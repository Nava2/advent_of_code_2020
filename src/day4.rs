use std::hash::Hash;
use std::cmp::PartialEq;
use std::num::ParseIntError;
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Hash, Eq, Copy)]
pub enum PassportField {
    BirthYear, // byr
    IssueYear, // iyr
    ExpirationYear, // eyr
    Height, // hgt
    HairColor, // hcl
    EyeColor, // ecl
    PassportId, // pid
    CountryId, // cid
    Unknown,
}

impl PassportField {
    pub fn parse(str_pair: &str, validate: bool) -> Result<PassportField, (PassportField, String)> {
        let pair = str_pair.split(PAIR_SEPARATOR).collect::<Vec<&str>>();
        let key = pair[0];
        
        let field = match key {
            "byr" => Ok(PassportField::BirthYear),
            "iyr" => Ok(PassportField::IssueYear),
            "eyr" => Ok(PassportField::ExpirationYear),
            "hgt" => Ok(PassportField::Height),
            "hcl" => Ok(PassportField::HairColor),
            "ecl" => Ok(PassportField::EyeColor),
            "pid" => Ok(PassportField::PassportId),
            "cid" => Ok(PassportField::CountryId),
            _ => Err((PassportField::Unknown, format!("Unknown key={:?}", key)))
        };
        
        if validate {
            let value = pair[1];
            field?.validate(value)
        }
        else { 
            field
        }
    }

    pub fn validate(&self, value: &str) -> Result<PassportField, (PassportField, String)> {
        match self {
            &PassportField::BirthYear | &PassportField::IssueYear | &PassportField::ExpirationYear => {
                let year = self.parse_u32(value)?;

                let range = match self {
                    PassportField::BirthYear => 1920..2003,
                    PassportField::IssueYear => 2010..2021,
                    PassportField::ExpirationYear => 2020..2031,
                    _ => unreachable!()
                };

                if !range.contains(&year) {
                    return self.error(format!("Invalid year range value={:?} !in {:?}", year, range))
                }
            },
            &PassportField::Height => {
                let value_len = value.chars().count();
                if value_len <= 2 {
                    return self.error(format!("Invalid value={:?}", value))
                }

                let cm_tag = value.chars().skip(value_len - 2).collect::<String>();
                let range = match &cm_tag[..] {
                    "in" => 59..77,
                    "cm" => 150..194,
                    _ => return self.error(format!("Unknown unit value={:?}", value))
                };

                let chars = value.chars().take(value_len - 2).collect::<String>();
                let height = self.parse_u32(&chars)?;

                if !range.contains(&height) {
                    return self.error(format!("Invalid height={:?} !in {:?}{}", value, range, cm_tag))
                }    
            },

            &PassportField::HairColor => {
                if value.chars().count() != 7 {
                    return self.error(format!("Malformed input={:?}", value))
                }

                let first_char = value.chars().next().unwrap();
                if first_char != '#' {
                    return self.error(format!("Malformed input={:?}", value))
                }

                for c in value.chars().skip(1) {
                    if !((c >= '0' && c <= '9') || (c >= 'a' && c <= 'f')) {
                        return self.error(format!("Malformed input={:?}", value))                        
                    }
                }
            }

            _ => todo!()
        }

        Ok(*self)
    }

    fn error(&self, message: String) -> Result<PassportField, (PassportField, String)> {
        Err((*self, message))
    }

    fn parse_u32(&self, value: &str) -> Result<u32, (PassportField, String)> {
        value.parse::<u32>()
            .map_err(|_| (*self, format!("Failed to parse u32 value={:?}", value)))
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

        for field in line.split_whitespace().map(|pair| PassportField::parse(pair, false)) {
            match field {
                Ok(f) => {
                    current_fields.insert(f);
                },
                Err((field, message)) => { println!("{:?} -> {:?}", field, message) },
            };
        }
    }

    result.push(current_fields);
    result
}

#[aoc(day4, part1)]
pub fn solve_part1(passports: &[HashSet<PassportField>]) -> usize {
    passports.iter()
    .filter(|&passport| {
        let passport_len = passport.len();
        passport_len == 8 || (passport_len == 7 && !passport.contains(&PassportField::CountryId))
    })
    .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn passport_fields_height() {
        assert_eq!(PassportField::parse("hgt:20", true), Err((PassportField::Height, String::from("Invalid value=\"20\""))));
        assert_eq!(PassportField::parse("hgt:200", true), Err((PassportField::Height, String::from("Unknown unit value=\"200\""))));
        assert_eq!(PassportField::parse("hgt:aaa", true), Err((PassportField::Height, String::from("Unknown unit value=\"aaa\""))));
        assert_eq!(PassportField::parse("hgt:aaacm", true), Err((PassportField::Height, String::from("Failed to parse u32 value=\"aaa\""))));
        
        assert_eq!(PassportField::parse("hgt:200cm", true), Err((PassportField::Height, String::from("Invalid height=\"200cm\" !in 150..194cm"))));
        assert_eq!(PassportField::parse("hgt:77in", true), Err((PassportField::Height, String::from("Invalid height=\"77in\" !in 59..77in"))));

        assert_eq!(PassportField::parse("hgt:150cm", true), Ok(PassportField::Height));
        assert_eq!(PassportField::parse("hgt:193cm", true), Ok(PassportField::Height));
        assert_eq!(PassportField::parse("hgt:59in", true), Ok(PassportField::Height));
        assert_eq!(PassportField::parse("hgt:76in", true), Ok(PassportField::Height));
    }

    #[test]
    fn passport_fields_birthyear() {
        assert_eq!(PassportField::parse("byr:aa", true), Err((PassportField::BirthYear, String::from("Failed to parse u32 value=\"aa\""))));
        assert_eq!(PassportField::parse("byr:20", true), Err((PassportField::BirthYear, String::from("Invalid year range value=20 !in 1920..2003"))));
        assert_eq!(PassportField::parse("byr:2003", true), Err((PassportField::BirthYear, String::from("Invalid year range value=2003 !in 1920..2003"))));
        assert_eq!(PassportField::parse("byr:2020", true), Err((PassportField::BirthYear, String::from("Invalid year range value=2020 !in 1920..2003"))));

        assert_eq!(PassportField::parse("byr:1920", true), Ok(PassportField::BirthYear));
        assert_eq!(PassportField::parse("byr:2002", true), Ok(PassportField::BirthYear));
    }

    #[test]
    fn passport_fields_issueyear() {
        assert_eq!(PassportField::parse("iyr:aa", true), Err((PassportField::IssueYear, String::from("Failed to parse u32 value=\"aa\""))));
        assert_eq!(PassportField::parse("iyr:20", true), Err((PassportField::IssueYear, String::from("Invalid year range value=20 !in 2010..2021"))));
        assert_eq!(PassportField::parse("iyr:2009", true), Err((PassportField::IssueYear, String::from("Invalid year range value=2009 !in 2010..2021"))));
        assert_eq!(PassportField::parse("iyr:2021", true), Err((PassportField::IssueYear, String::from("Invalid year range value=2021 !in 2010..2021"))));

        assert_eq!(PassportField::parse("iyr:2010", true), Ok(PassportField::IssueYear));
        assert_eq!(PassportField::parse("iyr:2020", true), Ok(PassportField::IssueYear));
    }

    #[test]
    fn passport_fields_expirationyear() {
        assert_eq!(PassportField::parse("eyr:aa", true), Err((PassportField::ExpirationYear, String::from("Failed to parse u32 value=\"aa\""))));
        assert_eq!(PassportField::parse("eyr:20", true), Err((PassportField::ExpirationYear, String::from("Invalid year range value=20 !in 2020..2031"))));
        assert_eq!(PassportField::parse("eyr:2019", true), Err((PassportField::ExpirationYear, String::from("Invalid year range value=2019 !in 2020..2031"))));
        assert_eq!(PassportField::parse("eyr:2031", true), Err((PassportField::ExpirationYear, String::from("Invalid year range value=2031 !in 2020..2031"))));

        assert_eq!(PassportField::parse("eyr:2020", true), Ok(PassportField::ExpirationYear));
        assert_eq!(PassportField::parse("eyr:2030", true), Ok(PassportField::ExpirationYear));
    }

    #[test]
    fn passport_fields_haircolor() {
        assert_eq!(PassportField::parse("hcl:", true), Err((PassportField::HairColor, String::from("Malformed input=\"\""))));
        assert_eq!(PassportField::parse("hcl:aaaaaa", true), Err((PassportField::HairColor, String::from("Malformed input=\"aaaaaa\""))));
        assert_eq!(PassportField::parse("hcl:#", true), Err((PassportField::HairColor, String::from("Malformed input=\"#\""))));
        assert_eq!(PassportField::parse("hcl:#a", true), Err((PassportField::HairColor, String::from("Malformed input=\"#a\""))));
        assert_eq!(PassportField::parse("hcl:#aaaaa", true), Err((PassportField::HairColor, String::from("Malformed input=\"#aaaaa\""))));
        assert_eq!(PassportField::parse("hcl:#aaaaaq", true), Err((PassportField::HairColor, String::from("Malformed input=\"#aaaaaq\""))));

        assert_eq!(PassportField::parse("hcl:#aaa000", true), Ok(PassportField::HairColor));
        assert_eq!(PassportField::parse("hcl:#fff999", true), Ok(PassportField::HairColor));
    }
}