
use std::collections::HashMap;
use std::ops::RangeInclusive;

#[derive(Debug, PartialEq, Clone)]
pub struct Rule {
    class: String,   
    ranges: Vec<RangeInclusive<u32>>,
}

impl Rule {
    fn parse(rule_line: &str) -> Rule {
        let class_ranges_pair = rule_line.split(':').collect::<Vec<_>>();
        let class_name = class_ranges_pair[0];
        let ranges = class_ranges_pair[1].split("or")
            .map(|v| v.trim())
            .map(|v| {
                let numbers = v.split('-')
                    .map(|n| n.parse::<u32>().unwrap())
                    .collect::<Vec<_>>();
                numbers[0]..=numbers[1]
            })
            .collect::<Vec<_>>();

        Rule {
            class: class_name.to_owned(),
            ranges,
        }
    }

    fn is_valid_field(&self, value: &u32) -> bool {
        is_in_ranges(&self.ranges, value)
    }
}

#[derive(Debug, PartialEq)]
pub struct Ticket {
    fields: Vec<u32>,
}

impl Ticket {
    fn parse(fields_line: &str) -> Ticket {
        Ticket {
            fields: fields_line.split(',')
                .filter(|v| !v.is_empty())
                .map(|v| v.trim().parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Notes {
    rules: Vec<Rule>,
    my_ticket: Ticket,
    tickets: Vec<Ticket>
}

impl Notes {
    fn parse(input: &str) -> Notes {
        let mut lines_iter = input.lines();

        let mut rules = Vec::new();
    
        while let Some(class_line) = lines_iter.next() {
            if class_line.is_empty() {
                break;
            }
    
            rules.push(Rule::parse(class_line));
        }
    
        lines_iter.next(); // skip "your ticket:"

        let my_ticket = Ticket::parse(lines_iter.next().unwrap());
    
        lines_iter.next(); // skip empty line
        lines_iter.next(); // skip nearby tickets:
    
        let tickets = lines_iter.map(|l| Ticket::parse(l)).collect::<Vec<_>>();
        
        Notes {
            rules,
            my_ticket,
            tickets,
        }
    }
}

#[aoc_generator(day16)]
pub fn input_generator(input: &str) -> Box<Notes> {
    Box::new(Notes::parse(input))
}

fn merge_ranges(lhs: &RangeInclusive<u32>, rhs: &RangeInclusive<u32>) -> RangeInclusive<u32> {
    RangeInclusive::new(
        std::cmp::min(*lhs.start(), *rhs.start()),
        std::cmp::max(*lhs.end(), *rhs.end())
    )
}

// collapses ranges into a sorted list
fn collapse_ranges(rules: &[Rule]) -> Vec<RangeInclusive<u32>> {
    rules.iter()
        .fold(Vec::<RangeInclusive<u32>>::new(), |mut acc, rule| {
            if acc.is_empty() {
                return rule.ranges.clone() // avoid case of empty
            }

            for range in rule.ranges.iter() {
                match acc.binary_search_by_key(&range.start(), |r| r.start()) {
                    Ok(idx) => {
                        // found the match at index, starts match explicitly :thinking:
                        let a_range = &acc[idx];
                        acc[idx] = merge_ranges(a_range, range);
                    },
                    Err(idx) => {
                        // index is _always_ denoting a value that is either in the prev or to be pushed to the end
                        if idx == 0 {
                            // thus the first is always special
                            acc.insert(0, range.clone());
                        }
                        else {
                            // last element is special due to insert @ last index doesnt' quite behave
                            let prev_range = &acc[idx - 1];
                            if prev_range.contains(&range.start()) {
                                acc[idx - 1] = merge_ranges(prev_range, range)
                            }
                            else {
                                // append it to the end then
                                acc.insert(idx, range.clone()); 
                            }
                        }
                    },
                }
            }

            acc
        })
}

fn is_in_ranges(ranges: &[RangeInclusive<u32>], value: &u32) -> bool {
    match ranges.binary_search_by_key(&value, |r| r.start()) {
        Ok(_) => true,
        Err(idx) => {
            // idx always points to prev
            if idx == 0 {
                false
            }
            else {
                let prev = &ranges[idx - 1];
                prev.contains(value)
            }
        }
    }
}

#[aoc(day16, part1)]
pub fn solve_part1(notes: &Notes) -> u64 {
    let collapsed_ranges = collapse_ranges(&notes.rules);

    notes.tickets.iter()
        .flat_map(|t| t.fields.iter().filter(|f| !is_in_ranges(&collapsed_ranges, f)))
        .fold(0u64, |a, v| a + (*v as u64))
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}

#[aoc(day16, part2)]
pub fn solve_part2(notes: &Notes) -> u64 {
    let collapsed_ranges = collapse_ranges(&notes.rules);

    let rules_n = notes.rules.len();
    let rules_by_name = {
        let mut map = HashMap::<&str, Rule>::with_capacity(rules_n);
        for rule in &notes.rules {
            map.insert(&rule.class, rule.clone());
        }
        map
    };
    let mut remaining_rules = notes.rules.iter().map(|r| &r.class[..]).collect::<Vec<_>>();

    let mut rules_map = HashMap::<&str, usize>::with_capacity(rules_n);

    // fields is now all the values in the tickets per column
    let fields = transpose(
        notes.tickets.iter()
            .filter(|t| t.fields.iter().all(|f| is_in_ranges(&collapsed_ranges, f)))
            .map(|t| t.fields.clone())
            .collect::<Vec<_>>()
    );

    let mut remaining_fields = (0..fields.len()).collect::<Vec<usize>>();

    while rules_map.len() != rules_n {
        remaining_fields.retain(|field_idx| {
            let current_fields = &fields[*field_idx];
            
            let matching_rules = remaining_rules.iter()
                .enumerate()
                .map(|(i, n)| (i, &rules_by_name[n]))
                .filter(|(_, r)| current_fields.iter().all(|f| r.is_valid_field(f)))
                .collect::<Vec<_>>();

            if matching_rules.len() == 1 {
                // found a single matching rule
                let (r_idx, rule) = matching_rules[0];

                rules_map.insert(&rule.class, *field_idx);
                remaining_rules.remove(r_idx);

                false
            }
            else {
                // more than one or no matches => not the _right_ rule
                true
            }
        });
    }

    rules_map.iter()
        .filter_map(|(k, v)| if k.starts_with("departure") { Some(v) } else { None })
        .map(|idx| notes.my_ticket.fields[*idx])
        .fold(1u64, |a, v| a * (v as u64))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day16_solve_part2_given() {
        let notes = Notes::parse(
                "departure class: 0-1 or 4-19\n\
                departure row: 0-5 or 8-19\n\
                seat: 0-13 or 16-19\n\
                \n\
                your ticket:\n\
                11,12,13\n\
                \n\
                nearby tickets:\n\
                3,9,18\n\
                15,1,5\n\
                5,14,9");
        assert_eq!(11 * 12, solve_part2(&notes));
    }

    #[test]
    fn solve_part1_given() {
        let notes = Notes::parse(
                "class: 1-3 or 5-7\n\
                row: 6-11 or 33-44\n\
                seat: 13-40 or 45-50\n\
                \n\
                your ticket:\n\
                7,1,14\n\
                \n\
                nearby tickets:\n\
                7,3,47\n\
                40,4,50\n\
                55,2,20\n\
                38,6,12");
        assert_eq!(71, solve_part1(&notes));
    }

    #[test]
    fn parse_rule() {
        assert_eq!(
            Rule { 
                class: "class".to_owned(),  
                ranges: vec![1..=3, 5..=7],
            }, 
            Rule::parse("class: 1-3 or 5-7"));
    }

    #[test]
    fn parse_ticket() {
        assert_eq!(
            Ticket {
                fields: vec![89, 137, 223, 97, 61],
            }, 
            Ticket::parse("89,137,223,97,61"));
    }

    #[test]
    fn parse_notes() {
        assert_eq!(
            Notes {
                rules: vec![
                    Rule {
                        class: "location".to_owned(),
                        ranges: vec![28..=184, 203..=952],
                    },
                    Rule {
                        class: "station".to_owned(),
                        ranges: vec![43..=261, 283..=958],
                    },
                ],
                my_ticket: Ticket { fields: vec![89, 137, 223, 97, 61], },
                tickets: vec![
                    Ticket { fields: vec![170, 218, 811, 107, 747], }, 
                    Ticket { fields: vec![683, 727, 850, 596, 125], }, 
                ],
            }, 
            Notes::parse(
                "location: 28-184 or 203-952\n\
                station: 43-261 or 283-958\n\
                \n\
                your ticket:\n\
                89,137,223,97,61\n\
                \n\
                nearby tickets:\n\
                170,218,811,107,747\n\
                683,727,850,596,125"));
    }

    #[test]
    fn collapse_ranges_given() {
        let notes = Notes::parse(
                "location: 28-184 or 203-952\n\
                station: 43-261 or 283-958\n\
                o: 0-21 or 35-100\n\
                b: 999-1048 or 1048-1111\n\
                \n\
                your ticket:\n\
                89,137,223,97,61\n\
                \n\
                nearby tickets:\n\
                170,218,811,107,747\n\
                683,727,850,596,125");
        assert_eq!(
            vec![0..=21, 28..=261, 203..=958, 999..=1111],
            collapse_ranges(&notes.rules),
        );
    }

    #[test]
    fn is_in_ranges_given() {
        let rule = Rule::parse("location: 5-20 or 40-50");
        let ranges = &rule.ranges;

        assert_eq!(true, is_in_ranges(ranges, &5));
        assert_eq!(true, is_in_ranges(ranges, &20));
        assert_eq!(true, is_in_ranges(ranges, &40));
        assert_eq!(true, is_in_ranges(ranges, &50));

        assert_eq!(false, is_in_ranges(ranges, &4));
        assert_eq!(false, is_in_ranges(ranges, &21));
        assert_eq!(false, is_in_ranges(ranges, &39));
        assert_eq!(false, is_in_ranges(ranges, &51));
    }
}