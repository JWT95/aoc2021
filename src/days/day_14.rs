use crate::common::read_input;
use anyhow::Result;
use recap::Recap;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize, PartialEq, Recap, Hash, Eq, Clone)]
#[recap(regex = r#"^(?P<pair>..) -> (?P<insert>.)"#)]
struct Rule {
    pair: String,
    insert: char,
}

#[derive(Debug, Deserialize, PartialEq, Recap, Hash, Eq, Clone)]
#[recap(regex = r#"(?P<left>.)(?P<right>.)"#)]
struct Pair {
    left: char,
    right: char,
}

fn apply_step(start_pairs: HashMap<Pair, u64>, rules: &HashMap<Pair, char>) -> HashMap<Pair, u64> {
    let mut new_pairs = HashMap::new();
    for pair in start_pairs {
        // Find the rule that applies
        let rule = rules.get(&pair.0).unwrap();

        // Add the same number to each new thing
        let left_child = Pair {
            left: pair.0.left,
            right: *rule,
        };
        let right_child = Pair {
            left: *rule,
            right: pair.0.right,
        };

        *new_pairs.entry(left_child).or_insert(0) += pair.1;
        *new_pairs.entry(right_child).or_insert(0) += pair.1;
    }

    new_pairs
}

fn part_one(start: String, rules: &HashMap<Pair, char>) {
    let mut pairs = HashMap::new();
    for i in 0..start.len() - 1 {
        *pairs
            .entry(Pair {
                left: start.chars().nth(i).unwrap(),
                right: start.chars().nth(i + 1).unwrap(),
            })
            .or_insert(0) += 1;
    }

    for _i in 0..40 {
        pairs = apply_step(pairs, rules);
    }

    let prevalence = calculate_prevalence(start, pairs);
    let min = prevalence.values().min().unwrap();
    let max = prevalence.values().max().unwrap();

    println!("{:?}", max - min);
}

fn calculate_prevalence(start: String, pairs: HashMap<Pair, u64>) -> HashMap<char, u64> {
    let mut counts = HashMap::new();
    for (pair, prevalence) in pairs {
        *counts.entry(pair.left).or_insert(0) += prevalence;
        *counts.entry(pair.right).or_insert(0) += prevalence;
    }

    // Add one to the start and end letter then divide by 2
    *counts.get_mut(&start.chars().nth(0).unwrap()).unwrap() += 1;
    *counts.get_mut(&start.chars().last().unwrap()).unwrap() += 1;

    counts.into_iter().map(|x| (x.0, x.1 / 2)).collect()
}

pub fn day_14() -> Result<()> {
    let input: Vec<String> = read_input("input/day_14.txt")?.collect();

    let start = input[0].clone();

    let rules: HashMap<_, _> = input[2..]
        .iter()
        .map(|x| x.parse::<Rule>().unwrap())
        .map(|x| (x.pair.parse::<Pair>().unwrap(), x.insert))
        .collect();

    part_one(start, &rules);

    Ok(())
}
