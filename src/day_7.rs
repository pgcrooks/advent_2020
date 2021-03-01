use std::error::Error;
use std::cmp;
use std::fmt;
use std::fs;

use regex::Regex;

use advent::Config;

#[derive(Debug, Clone)]
pub struct Contents {
    pub colour: String,
    pub count: i32,
}

impl cmp::PartialEq for Contents {
    fn eq(&self, other: &Self) -> bool {
        self.colour == other.colour && self.count == other.count
    }
}

#[derive(Default, Clone)]
pub struct Rule {
    pub bag_colour: String,
    pub inner_bags: Vec<Contents>,
}

impl fmt::Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Rule: bag_colour=\"{}\" inner_bags={:?}",
            self.bag_colour, self.inner_bags
        )
    }
}

fn parse_rule(input: &str) -> Rule {
    let outer_re = Regex::new(r"^(.*) bags contain (.*)\.$").unwrap();
    let inner_re = Regex::new(r"^([0-9]+) (.*) bags?$").unwrap();

    let mut rule = Rule::default();

    let re_result = outer_re.captures(input);
    if re_result.is_some() {
        let outer_group = re_result.unwrap();

        rule.bag_colour = outer_group[1].to_string();

        let inner_strings: Vec<String> = outer_group[2].split(", ").map(|x| x.to_string()).collect();
        for inner in inner_strings {
            let inner_re_result = inner_re.captures(&inner);
            if inner_re_result.is_some() {
                let inner_group = inner_re_result.unwrap();

                let inner_bag = Contents {
                    colour: inner_group[2].to_string(),
                    count: inner_group[1].parse().unwrap_or_default()
                };
                rule.inner_bags.push(inner_bag);
            }
        }

        // println!("{}", rule);
    } else {
        println!("[ERROR] Could not parse rule: \"{}\"", input);
    }

    return rule;
}

fn parse_rules(rules: Vec<&str>) -> Vec<Rule> {
    let mut parsed_rules: Vec<Rule> = vec![];

    for line in rules {
        if line.len() > 1 {
            parsed_rules.push(parse_rule(line));
        }
    }

    println!("\nParsed {} rules.", parsed_rules.len());

    return parsed_rules;
}

fn search_for_bag(rules: &Vec<Rule>, search_colour: &str) -> Vec<String> {
    let mut matching_bags: Vec<String> = vec![];

    for rule in rules {
        for inner_bag in &rule.inner_bags {
            if inner_bag.colour == search_colour {
                // Found a match!
                // println!("Matched {} in {}", search_colour, rule);
                matching_bags.push(rule.bag_colour.clone());

                // Can the outer bag also be contained within another?
                let mut temp = search_for_bag(&rules, &rule.bag_colour);
                matching_bags.append(&mut temp);
            }
        }
    }

    return matching_bags;
}

fn count_bags(rules: &Vec<Rule>, search_colour: &str) -> usize {
    let mut matching_bags: Vec<String> = search_for_bag(rules, search_colour);

    // Remove duplicate outer bags
    matching_bags.sort();
    matching_bags.dedup();

    return matching_bags.len();
}

fn count_inner_bags(rules: &Vec<Rule>, search_colour: &str) -> i32 {
    let mut count = 0;

    // println!("Searching for {}", search_colour);
    for rule in rules {
        if rule.bag_colour == search_colour {
            // println!("  found it in {}", rule);

            // Look through the bags it continues
            for inner in &rule.inner_bags {
                // println!("Adding {} {} bags.", inner.count, inner.colour);
                count += inner.count;
                count += count_inner_bags(rules, &inner.colour) * inner.count;
            }
        }
    }

    return count;
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    let rules: Vec<&str> = contents.split('\n').collect();

    let search_colour: &str = "shiny gold";

    let rule_list = parse_rules(rules);

    println!(
        "Number of bags that eventually contain a {} bag is {}",
        search_colour, count_bags(&rule_list, search_colour)
    );

    println!(
        "Number of bags that a {} bag contains is {}",
        search_colour, count_inner_bags(&rule_list, search_colour)
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_parse_rules {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (input, expected) = $value;
                let actual = parse_rule(input);
                assert_eq!(expected.bag_colour, actual.bag_colour);
                assert_eq!(expected.inner_bags.len(), actual.inner_bags.len());

                // Compare inner bags
                let it = expected.inner_bags.iter().zip(actual.inner_bags.iter());
                for (_i, (x, y)) in it.enumerate() {
                    assert_eq!(x, y);
                }
            }
        )*
        }
    }
    test_parse_rules! {
        rule_0: (
            "light red bags contain 1 bright white bag, 2 muted yellow bags.",
            Rule{
                bag_colour: "light red".to_string(),
                inner_bags: vec![
                    Contents{colour: "bright white".to_string(), count: 1},
                    Contents{colour: "muted yellow".to_string(), count: 2}
                ]
            }
        ),
        rule_1: (
            "dark orange bags contain 3 bright white bags, 4 muted yellow bags.",
            Rule{
                bag_colour: "dark orange".to_string(),
                inner_bags: vec![
                    Contents{colour: "bright white".to_string(), count: 3},
                    Contents{colour: "muted yellow".to_string(), count: 4}
                ]
            }
        ),
        rule_2: (
            "bright white bags contain 1 shiny gold bag.",
            Rule{
                bag_colour: "bright white".to_string(),
                inner_bags: vec![
                    Contents{colour: "shiny gold".to_string(), count: 1}
                ]
            }
        ),
        rule_3: (
            "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.",
            Rule{
                bag_colour: "muted yellow".to_string(),
                inner_bags: vec![
                    Contents{colour: "shiny gold".to_string(), count: 2},
                    Contents{colour: "faded blue".to_string(), count: 9}
                ]
            }
        ),
        rule_4: (
            "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.",
            Rule{
                bag_colour: "shiny gold".to_string(),
                inner_bags: vec![
                    Contents{colour: "dark olive".to_string(), count: 1},
                    Contents{colour: "vibrant plum".to_string(), count: 2}
                ]
            }
        ),
        rule_5: (
            "dark olive bags contain 3 faded blue bags, 4 dotted black bags.",
            Rule{
                bag_colour: "dark olive".to_string(),
                inner_bags: vec![
                    Contents{colour: "faded blue".to_string(), count: 3},
                    Contents{colour: "dotted black".to_string(), count: 4}
                ]
            }
        ),
        rule_6: (
            "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.",
            Rule{
                bag_colour: "vibrant plum".to_string(),
                inner_bags: vec![
                    Contents{colour: "faded blue".to_string(), count: 5},
                    Contents{colour: "dotted black".to_string(), count: 6}
                ]
            }
        ),
        rule_7: (
            "faded blue bags contain no other bags.",
            Rule{
                bag_colour: "faded blue".to_string(),
                inner_bags: vec![]
            }
        ),
        rule_8: (
            "dotted black bags contain no other bags.",
            Rule{
                bag_colour: "dotted black".to_string(),
                inner_bags: vec![]
            }
        ),
    }

    #[test]
    fn test_count_bags_will_return_correct_count() {
        let test_data = vec![
            "light red bags contain 1 bright white bag, 2 muted yellow bags.",
            "dark orange bags contain 3 bright white bags, 4 muted yellow bags.",
            "bright white bags contain 1 shiny gold bag.",
            "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.",
            "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.",
            "dark olive bags contain 3 faded blue bags, 4 dotted black bags.",
            "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.",
            "faded blue bags contain no other bags.",
            "dotted black bags contain no other bags.",
        ];

        let parsed_rules = parse_rules(test_data);

        assert_eq!(4, count_bags(&parsed_rules, "shiny gold"));
    }

    #[test]
    fn test_count_inner_bags_returns_count() {
        let test_data = vec![
            "shiny gold bags contain 2 dark red bags.",
            "dark red bags contain 2 dark orange bags.",
            "dark orange bags contain 2 dark yellow bags.",
            "dark yellow bags contain 2 dark green bags.",
            "dark green bags contain 2 dark blue bags.",
            "dark blue bags contain 2 dark violet bags.",
            "dark violet bags contain no other bags.",
        ];

        let parsed_rules = parse_rules(test_data);

        assert_eq!(126, count_inner_bags(&parsed_rules, "shiny gold"));
    }
}
