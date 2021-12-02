use crate::common::read_input;
use anyhow::Result;

fn part_one(inputs: &[u32]) -> u32 {
    let mut changes = 0;
    for i in 0..(inputs.len() - 1) {
        if inputs[i] < inputs[i + 1] {
            changes += 1;
        }
    }

    changes
}

fn part_two(inputs: &[u32]) -> u32 {
    let mut changes = 0;
    for i in 0..(inputs.len() - 3) {
        if window(i, inputs) < window(i + 1, inputs) {
            changes += 1;
        }
    }

    changes
}

fn window(index: usize, inputs: &[u32]) -> u32 {
    let mut sum = 0;
    for i in 0..3 {
        sum += inputs[index + i];
    }
    sum
}

pub fn day_01() -> Result<()> {
    let inputs: Vec<u32> = read_input("input/day_01.txt")?
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    println!("{:?}", part_one(&inputs));
    println!("{:?}", part_two(&inputs));

    Ok(())
}
