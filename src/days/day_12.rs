use crate::common::read_input;
use anyhow::Result;
use recap::Recap;
use serde::Deserialize;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Deserialize, PartialEq, Recap)]
#[recap(regex = r#"(?P<left>\S+)\-(?P<right>\S+)"#)]
struct Input {
    left: String,
    right: String,
}

pub fn explore_part_one(path_so_far: Vec<String>, caves: &HashMap<String, HashSet<String>>) -> u32 {
    let mut routes = 0;

    let current_cave = path_so_far.last().unwrap();

    if current_cave == "end" {
        return 1;
    }

    for cave in caves.get(current_cave).unwrap() {
        if !(path_so_far.contains(cave) && cave.chars().all(|c| c.is_ascii_lowercase())) {
            let mut new_path_so_far = path_so_far.clone();
            new_path_so_far.push(cave.clone());
            routes += explore_part_one(new_path_so_far, caves)
        }
    }

    routes
}

pub fn explore_part_two(path_so_far: Vec<String>, caves: &HashMap<String, HashSet<String>>) -> u32 {
    let mut routes = 0;

    let current_cave = path_so_far.last().unwrap();

    if current_cave == "end" {
        return 1;
    }

    for cave in caves.get(current_cave).unwrap() {
        // If there are duplicates in the path already, don't continue
        if cave == "start" {
        } else if contains_double_lower_case(&path_so_far) {
            if !(path_so_far.contains(cave) && cave.chars().all(|c| c.is_ascii_lowercase())) {
                let mut new_path_so_far = path_so_far.clone();
                new_path_so_far.push(cave.clone());
                routes += explore_part_two(new_path_so_far, caves)
            }
        } else {
            let mut new_path_so_far = path_so_far.clone();
            new_path_so_far.push(cave.clone());
            routes += explore_part_two(new_path_so_far, caves)
        }
    }

    routes
}

fn contains_double_lower_case(path_so_far: &Vec<String>) -> bool {
    let lower_case: Vec<String> = path_so_far
        .iter()
        .filter(|x| x.chars().all(|c| c.is_ascii_lowercase()))
        .cloned()
        .collect();

    for cave in lower_case.clone() {
        if lower_case.iter().filter(|x| **x == cave).count() > 1 {
            return true;
        }
    }

    false
}

pub fn day_12() -> Result<()> {
    let inputs: Vec<Input> = read_input("input/day_12.txt")?
        .map(|x| x.parse::<Input>().unwrap())
        .collect();

    let mut cave_system: HashMap<String, HashSet<String>> = HashMap::new();

    for input in inputs {
        let entry = cave_system
            .entry(input.left.clone())
            .or_insert(HashSet::new());
        entry.insert(input.right.clone());

        let entry = cave_system.entry(input.right).or_insert(HashSet::new());
        entry.insert(input.left);
    }
    println!("{:?}", cave_system);
    println!(
        "{:?}",
        explore_part_one(vec![String::from("start")], &cave_system)
    );
    println!(
        "{:?}",
        explore_part_two(vec![String::from("start")], &cave_system)
    );

    Ok(())
}
