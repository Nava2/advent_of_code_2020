use std::collections::{ HashMap, HashSet, VecDeque };
use std::cell::{RefCell, RefMut};
use regex::Regex;

/*
light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.
*/

const CONTAIN_KEY: &str = "contain";

const NO_RULES_KEY: &str = "no other bags.";

#[derive(Debug, Clone)]
pub struct BagRule {
    color: String,
    count: usize,
}

impl BagRule {
    fn parse_all(rules: &str) -> Vec<BagRule> {
        if rules == NO_RULES_KEY {
            return Vec::new();
        }

        lazy_static! {
            static ref RULES_RE: Regex = Regex::new(r"\s*(?P<count>\d+)\s+(?P<color>[\w\s]+)\s+bags?[\.,]\s*").unwrap();
        }

        RULES_RE.captures_iter(rules)
            .map(|cap| {
                let color = &cap["color"];
                let count = cap["count"].parse::<usize>().unwrap();    
                BagRule { color: color.to_owned(), count }
            })
            .collect()
    }
}

#[derive(Debug, Clone)]
pub struct Rule {
    color: String,
    rules: Vec<BagRule>,
}

impl Rule {
    fn parse(line: &str) -> Rule {
        let (color, rules) = {
            let parts = line.split(CONTAIN_KEY).map(|p| p.trim()).collect::<Vec<&str>>();
            debug_assert!(parts.len() == 2);

            (parts[0], parts[1])
        };

        let color = color.trim_end_matches(" bags");

        let rules = BagRule::parse_all(rules);

        Rule {
            color: color.to_owned(),
            rules,
        }
    }
}

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> HashMap<String, Rule> {
    let result = input.lines()
        .map(|line| Rule::parse(line))
        .map(|rule| (rule.color.clone(), rule))
        .collect::<HashMap<String, Rule>>();
    result
}

fn compute_contained_in(rules: &HashMap<String, Rule>) -> HashMap::<String, HashSet<String>> {
    let mut contained_in = HashMap::<String, RefCell<HashSet<String>>>::new();

    for (key, rule) in rules {
        rule.rules.iter().for_each(|r| {
            match contained_in.get_mut(&r.color) {
                Some(contained) => { contained.get_mut().insert(key.clone()); },
                None => { 
                    let mut new_set = HashSet::new();
                    new_set.insert(key.clone());

                    contained_in.insert(r.color.clone(), RefCell::new(new_set)); 
                },
            };
        })
    }

    contained_in.into_iter()
        .map(|(k, v)| (k, v.into_inner()))
        .collect::<HashMap<String, HashSet<String>>>()
}

fn compute_valid_bags(rules: &HashMap<String, Rule>) -> HashSet<String> {
    let contained_in = compute_contained_in(rules);

    let search_field = "shiny gold";
    let mut valid_bags = HashSet::<String>::new();

    let mut next_q = vec![search_field.to_owned()];

    let mut visited = HashSet::<String>::new();
    while let Some(color) = next_q.pop() {

        if color != search_field {
            valid_bags.insert(color.clone());
        }

        if !visited.insert(color.clone()) {
            continue;
        }

        if let Some(next_set) = contained_in.get(&color) {
            for next_c in next_set.iter().cloned() {
                next_q.push(next_c);
            }
        }
    }

    println!("Found {} answers = {:?}", valid_bags.len(), valid_bags);
    valid_bags
}

#[aoc(day7, part1)]
pub fn solve_part1(bag_graph: &HashMap<String, Rule>) -> usize {
    compute_valid_bags(bag_graph).len()
}

#[aoc(day7, part2)]
pub fn solve_part2(bag_graph: &HashMap<String, Rule>) -> usize {
    let marked_color = "shiny gold";

    let mut memoized = bag_graph.iter()
        .filter(|(_, r)| r.rules.is_empty())
        .map(|(k, _)| (k.clone(), 1 as usize))
        .collect::<HashMap<String, usize>>();

    let mut q = VecDeque::<&str>::new();
    q.push_back(marked_color);

    while let Some(color) = q.pop_front() {
        if memoized.contains_key(color) {
            continue;
        }

        let within = bag_graph.get(color).unwrap();
        let results = within.rules.iter()
            .filter_map(|r| memoized.get(&r.color).map(|s| r.count * s))
            .collect::<Vec<_>>();
        
        if results.len() == within.rules.len() {
            let sum = results.into_iter().sum::<usize>();
            memoized.insert(color.to_owned(), sum + 1);
        }
        else {
            let uncounted_colors = within.rules.iter()
                .filter_map(|r| {
                    if memoized.contains_key(&r.color) {
                        None 
                    } else { 
                        Some(&r.color) 
                    }
                })
                .collect::<Vec<_>>();

            for uncounted_color in uncounted_colors {
                q.push_back(uncounted_color);
            }

            q.push_back(color);
        }
    }

    *memoized.get(marked_color).unwrap() - 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_input_part1() {
        let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.\n\
            dark orange bags contain 3 bright white bags, 4 muted yellow bags.\n\
            bright white bags contain 1 shiny gold bag.\n\
            muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.\n\
            shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.\n\
            dark olive bags contain 3 faded blue bags, 4 dotted black bags.\n\
            vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.\n\
            faded blue bags contain no other bags.\n\
            dotted black bags contain no other bags.";

        let rules = input_generator(input);
        for (i, (color, rule)) in rules.iter().enumerate() {
            println!("{:>3} {:?} -> {:?}", i, color, rule);
        }

        let valid_bags = compute_valid_bags(&rules);

        assert_eq!(
            valid_bags, 
            vec!["bright white", "light red", "dark orange", "muted yellow"].iter()
                .cloned()
                .map(|s| s.to_owned())
                .collect::<HashSet<String>>());
    }

    #[test]
    fn given_input_part2() {
        let input = "shiny gold bags contain 2 dark red bags.\n\
        dark red bags contain 2 dark orange bags.\n\
        dark orange bags contain 2 dark yellow bags.\n\
        dark yellow bags contain 2 dark green bags.\n\
        dark green bags contain 2 dark blue bags.\n\
        dark blue bags contain 2 dark violet bags.\n\
        dark violet bags contain no other bags.";

        let rules = input_generator(input);
        for (i, (color, rule)) in rules.iter().enumerate() {
            println!("{:>3} {:?} -> {:?}", i, color, rule);
        }

        assert_eq!(solve_part2(&rules), 126);
    }
}
