use crate::common::read_input;
use anyhow::Result;
use std::collections::HashMap;

fn part_one(mut inputs: Vec<u32>) -> Result<()> {
    let s = inputs.sort();

    let median = inputs[inputs.len() / 2];

    let sum: i32 = inputs
        .iter()
        .map(|x| (*x as i32 - median as i32).abs())
        .sum();

    println!("{:?}", sum);

    Ok(())
}

fn part_two(mut inputs: Vec<u32>) -> Result<()> {
    let s = inputs.sort();

    let sum: u32 = inputs.iter().sum();

    let mean = (sum as f64 / inputs.len() as f64).round() as u64;

    // Take the lowest around the mean
    let mut winning_sum = (10 as u64).pow(10) as f64;
    for i in mean - 3..mean + 3 {
        let sum: f64 = inputs
            .iter()
            .map(|x| {
                let y = (*x as f64 - i as f64).abs();
                (0.5 * y as f64 * (y + 1 as f64) as f64).round()
            })
            .sum();

        if sum < winning_sum {
            winning_sum = sum;
        }
    }

    println!("{:?}", winning_sum);

    Ok(())
}

pub fn day_07() -> Result<()> {
    let mut inputs: Vec<u32> = read_input("input/day_07.txt")?
        .nth(0)
        .unwrap()
        .split(",")
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    part_one(inputs.clone());
    part_two(inputs.clone());

    Ok(())
}
