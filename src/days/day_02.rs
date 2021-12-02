use crate::common::read_input;
use anyhow::Result;
use std::str::FromStr;

enum Command {
    Forward(u32),
    Down(u32),
    Up(u32),
}

impl FromStr for Command {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut split = s.split(" ");
        let direction = split.nth(0).ok_or_else(|| anyhow::Error::msg("Failed"))?;
        let amount = split
            .nth(0)
            .ok_or_else(|| anyhow::Error::msg("Failed"))?
            .parse::<u32>()?;

        Ok(match direction {
            "forward" => Self::Forward(amount),
            "down" => Self::Down(amount),
            "up" => Self::Up(amount),
            _ => anyhow::bail!("Invalid direction"),
        })
    }
}

fn part_one(commands: &[Command]) -> u32 {
    let mut depth = 0;
    let mut forward = 0;
    for command in commands {
        match command {
            Command::Forward(amount) => forward += amount,
            Command::Down(amount) => depth += amount,
            Command::Up(amount) => depth -= amount,
        }
    }

    depth * forward
}

fn part_two(commands: &[Command]) -> u32 {
    let mut depth = 0;
    let mut aim = 0;
    let mut forward = 0;

    for command in commands {
        match command {
            Command::Forward(amount) => {
                forward += amount;
                depth += aim * amount
            }
            Command::Down(amount) => aim += amount,
            Command::Up(amount) => aim -= amount,
        }
    }

    depth * forward
}

pub fn day_02() -> Result<()> {
    let inputs: Vec<Command> = read_input("input/day_02.txt")?
        .map(|x| Command::from_str(&x).unwrap())
        .collect();

    println!("{:?}", part_one(&inputs));
    println!("{:?}", part_two(&inputs));

    Ok(())
}
