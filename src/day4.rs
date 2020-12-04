use std::hash::Hash;
use std::cmp::PartialEq;
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

lazy_static! {
    static ref VALID_COLORS: HashSet<&'static str> = {
        let mut valid_colors = HashSet::with_capacity(7);
        valid_colors.insert("amb");
        valid_colors.insert("blu");
        valid_colors.insert("brn");
        valid_colors.insert("gry");
        valid_colors.insert("grn");
        valid_colors.insert("hzl");
        valid_colors.insert("oth");

        valid_colors
    };
}

impl PassportField {
    pub fn parse(key: &str) -> Result<PassportField, String> {
        match key {
            "byr" => Ok(PassportField::BirthYear),
            "iyr" => Ok(PassportField::IssueYear),
            "eyr" => Ok(PassportField::ExpirationYear),
            "hgt" => Ok(PassportField::Height),
            "hcl" => Ok(PassportField::HairColor),
            "ecl" => Ok(PassportField::EyeColor),
            "pid" => Ok(PassportField::PassportId),
            "cid" => Ok(PassportField::CountryId),
            _ => Err(format!("Unknown key={:?}", key)),
        }
    }

    pub fn validate(&self, value: &str) -> Result<PassportField, String> {
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
                    return self.malformed_input(value)
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
                    return self.malformed_input(value)
                }

                let first_char = value.chars().next().unwrap();
                if first_char != '#' {
                    return self.malformed_input(value)
                }

                for c in value.chars().skip(1) {
                    if !((c >= '0' && c <= '9') || (c >= 'a' && c <= 'f')) {
                        return self.malformed_input(value)                   
                    }
                }
            },

            &PassportField::EyeColor => {
                if value.chars().count() != 3 {
                    return self.malformed_input(value)
                }

                if !VALID_COLORS.contains(value) {
                    return self.error(format!("Unsupported eye color={:?}", value))
                }
            },

            &PassportField::PassportId => {
                if value.chars().count() != 9 {
                    return self.malformed_input(value)
                }

                self.parse_u32(value)?;
            }

            &PassportField::CountryId => (), // no validation!

            &PassportField::Unknown => unreachable!(),
        }

        Ok(*self)
    }

    fn error(&self, message: String) -> Result<PassportField, String> {
        Err(format!("{:?}: {}", self, message))
    }

    fn malformed_input(&self, value: &str) -> Result<PassportField, String> {
        self.error(format!("Malformed input={:?}", value))
    }

    fn parse_u32(&self, value: &str) -> Result<u32, String> {
        value.parse::<u32>()
            .map_err(|_| (format!("{:?}: Failed to parse u32 value={:?}", self, value)))
    }
}

const PAIR_SEPARATOR: char = ':';

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<Vec<(String, String)>> {
    let mut result = Vec::new();

    let mut current_fields = Vec::<(String, String)>::with_capacity(8);
    for line in input.lines() {
        if line.chars().count() == 0 {
            result.push(current_fields);
            current_fields = Vec::with_capacity(8);
        }

        let pairs = line.split_whitespace()
            .map(|pair| {
                pair.split(PAIR_SEPARATOR).map(str::to_string).collect::<Vec<String>>()
            });

        for pair in pairs {
            current_fields.push((pair[0].clone(), pair[1].clone()));
        }
    }

    result.push(current_fields);
    result
}

#[aoc(day4, part1)]
pub fn solve_part1(passports: &[Vec<(String, String)>]) -> usize {
    passports.iter()
        .map(|fields| { 
            fields.iter()
                .map(|(field, _)| PassportField::parse(&field).unwrap())
                .collect::<Vec<PassportField>>() 
        })
        .filter(|passport| {
            let passport_len = passport.len();
            if passport_len == 7 {
                passport.iter().any(|f| f == &PassportField::CountryId)
            }
            else {
                passport_len == 8
            }
        })
        .count()
}

#[aoc(day4, part2)]
pub fn solve_part2(passports: &[Vec<(String, String)>]) -> usize {
    passports.iter()
        .map(|fields| { 
            fields.iter()
                .map(|(key, value)| {
                    let field = PassportField::parse(&key)?;
                    field.validate(value)
                })
                .collect::<Vec<Result<PassportField, String>>>() 
        })
        .filter_map(|passport| {
            let cleaned_fields = passport.into_iter()
                .filter_map(|r| {
                    match r {
                        Ok(PassportField::CountryId) => None, // ignore

                        Ok(f) => Some(f),

                        Err(message) => { 
                            println!("Failed to validate: {}", message); 
                            None 
                        }
                    }
                })
                .collect::<HashSet<PassportField>>();

            if cleaned_fields.len() == 7 { Some(cleaned_fields) } else { None }
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_validate_fail(field: PassportField, value: &str, error: &str) {
        assert_eq!(field.validate(value), Err(error.to_string()));
    }

    fn assert_validate_success(field: PassportField, value: &str) {
        assert_eq!(field.validate(value), Ok(field));
    }

    fn assert_parse_field(input: &str, field: PassportField) {
        assert_eq!(PassportField::parse(input), Ok(field));
    }

    #[test]
    fn passport_fields_height() {
        assert_parse_field("hgt", PassportField::Height);

        assert_validate_fail(PassportField::Height, "200", "Unknown unit value=\"200\"");
        assert_validate_fail(PassportField::Height, "aaa", "Unknown unit value=\"aaa\"");
        assert_validate_fail(PassportField::Height, "aaacm", "Failed to parse u32 value=\"aaa\"");
        
        assert_validate_fail(PassportField::Height, "200cm", "Invalid height=\"200cm\" !in 150..194cm");
        assert_validate_fail(PassportField::Height, "77in", "Invalid height=\"77in\" !in 59..77in");

        assert_validate_success(PassportField::Height, "150cm");
        assert_validate_success(PassportField::Height, "193cm");
        assert_validate_success(PassportField::Height, "59in");
        assert_validate_success(PassportField::Height, "76in");
    }

    #[test]
    fn passport_fields_birthyear() {
        assert_parse_field("byr", PassportField::BirthYear);

        assert_validate_fail(PassportField::BirthYear, "aa", "Failed to parse u32 value=\"aa\"");
        assert_validate_fail(PassportField::BirthYear, "20", "Invalid year range value=20 !in 1920..2003");
        assert_validate_fail(PassportField::BirthYear, "2003", "Invalid year range value=2003 !in 1920..2003");
        assert_validate_fail(PassportField::BirthYear, "2020", "Invalid year range value=2020 !in 1920..2003");

        assert_validate_success(PassportField::BirthYear, "1920");
        assert_validate_success(PassportField::BirthYear, "2002");
    }

    #[test]
    fn passport_fields_issueyear() {
        assert_parse_field("iyr", PassportField::IssueYear);

        assert_validate_fail(PassportField::IssueYear, "aa", "Failed to parse u32 value=\"aa\"");
        assert_validate_fail(PassportField::IssueYear, "20", "Invalid year range value=20 !in 2010..2021");
        assert_validate_fail(PassportField::IssueYear, "2009", "Invalid year range value=2009 !in 2010..2021");
        assert_validate_fail(PassportField::IssueYear, "2021", "Invalid year range value=2021 !in 2010..2021");

        assert_validate_success(PassportField::IssueYear, "2010");
        assert_validate_success(PassportField::IssueYear, "2015");
        assert_validate_success(PassportField::IssueYear, "2012");
        assert_validate_success(PassportField::IssueYear, "2020");
    }

    #[test]
    fn passport_fields_expirationyear() {
        assert_parse_field("eyr", PassportField::ExpirationYear);

        assert_validate_fail(PassportField::ExpirationYear, "aa", "Failed to parse u32 value=\"aa\"");
        assert_validate_fail(PassportField::ExpirationYear, "20", "Invalid year range value=20 !in 2020..2031");
        assert_validate_fail(PassportField::ExpirationYear, "2019", "Invalid year range value=2019 !in 2020..2031");
        assert_validate_fail(PassportField::ExpirationYear, "2031", "Invalid year range value=2031 !in 2020..2031");

        assert_validate_success(PassportField::ExpirationYear, "2020");
        assert_validate_success(PassportField::ExpirationYear, "2030");
    }

    #[test]
    fn passport_fields_haircolor() {
        assert_parse_field("hcl", PassportField::HairColor);

        assert_validate_fail(PassportField::HairColor, "", "Malformed input=\"\"");
        assert_validate_fail(PassportField::HairColor, "aaaaaa", "Malformed input=\"aaaaaa\"");
        assert_validate_fail(PassportField::HairColor, "#", "Malformed input=\"#\"");
        assert_validate_fail(PassportField::HairColor, "#a", "Malformed input=\"#a\"");
        assert_validate_fail(PassportField::HairColor, "#aaaaa", "Malformed input=\"#aaaaa\"");
        assert_validate_fail(PassportField::HairColor, "#aaaaaq", "Malformed input=\"#aaaaaq\"");

        assert_validate_success(PassportField::HairColor, "#aaa000");
        assert_validate_success(PassportField::HairColor, "#fff999");
    }

    #[test]
    fn passport_fields_eyecolor() {
        assert_parse_field("ecl", PassportField::EyeColor);

        assert_validate_fail(PassportField::EyeColor, "", "Malformed input=\"\"");
        assert_validate_fail(PassportField::EyeColor, "hzlhzl", "Malformed input=\"hzlhzl\"");
        assert_validate_fail(PassportField::EyeColor, "aa0", "Unsupported eye color=\"aa0\"");

        for color in VALID_COLORS.iter() {
            assert_validate_success(PassportField::EyeColor, color);
        }
    }

    #[test]
    fn passport_fields_passportid() {
        assert_parse_field("pid", PassportField::PassportId);

        assert_validate_fail(PassportField::PassportId, "", "Malformed input=\"\"");
        assert_validate_fail(PassportField::PassportId, "000", "Malformed input=\"000\"");
        assert_validate_fail(PassportField::PassportId, "0123456789", "Malformed input=\"0123456789\"");

        assert_validate_fail(PassportField::PassportId, "aaaaaaaaa", "Failed to parse u32 value=\"aaaaaaaaa\"");

        assert_validate_success(PassportField::PassportId, "000000001");
        assert_validate_success(PassportField::PassportId, "012345678");
        assert_validate_success(PassportField::PassportId, "087499704");
        assert_validate_success(PassportField::PassportId, "896056539");
        assert_validate_success(PassportField::PassportId, "545766238");
        assert_validate_success(PassportField::PassportId, "093154719");
    }
}