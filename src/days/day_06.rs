use crate::common::read_input;
use anyhow::Result;
use std::collections::HashMap;

fn play_one_day(fish: HashMap<u64, u64>) -> Result<HashMap<u64, u64>> {
    let mut next_day_fish = fish.clone();

    for i in 0..8 {
        next_day_fish.insert(i, fish[&(i + 1)]);
    }

    *next_day_fish.entry(6).or_insert(0) += fish[&0];
    next_day_fish.insert(8, fish[&0]);

    Ok(next_day_fish)
}

fn part_one(fish: HashMap<u64, u64>) -> Result<()> {
    let mut fish = fish;
    for i in 0..80 {
        fish = play_one_day(fish)?;
    }

    println!("{}", fish.values().sum::<u64>());

    Ok(())
}

fn part_two(fish: HashMap<u64, u64>) -> Result<()> {
    let mut fish = fish;
    for i in 0..256 {
        fish = play_one_day(fish)?;
    }

    println!("{}", fish.values().sum::<u64>());

    Ok(())
}

pub fn day_06() -> Result<()> {
    let inputs: Vec<u64> = read_input("input/day_06.txt")?
        .nth(0)
        .unwrap()
        .split(",")
        .map(|x| x.parse::<u64>().unwrap())
        .collect();

    println!("{:?}", inputs);

    let mut fish = HashMap::new();

    for input in inputs {
        *fish.entry(input).or_insert(0) += 1;
    }

    for i in 0..9 {
        fish.entry(i).or_insert(0);
    }

    part_one(fish.clone());
    part_two(fish);

    Ok(())
}
