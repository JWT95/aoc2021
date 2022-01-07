use crate::common::read_input;
use anyhow::Result;
use std::collections::HashMap;

static directions: [(isize, isize); 8] = [
    (-1, 0),
    (1, 0),
    (-1, 1),
    (1, -1),
    (0, 1),
    (0, -1),
    (1, 1),
    (-1, -1),
];

fn play_one_round(octopi: &mut HashMap<(isize, isize), isize>) -> usize {
    // Add one to each octopus
    for (_, score) in octopi.iter_mut() {
        *score += 1;
    }

    // Find all octopus with score 10. If there are any pop them
    let mut pops = 0;
    while octopi.values().filter(|x| **x == 10).count() > 0 {
        // Find such octopi
        let coords: Vec<(isize, isize)> = octopi
            .iter()
            .filter(|x| *x.1 == 10)
            .map(|x| x.0.clone())
            .collect();

        pops += coords.len();

        for coord in coords {
            for neighbour in directions.iter().map(|x| (x.0 + coord.0, x.1 + coord.1)) {
                if let Some(x) = octopi.get_mut(&neighbour) {
                    match x {
                        0 => {}
                        10 => {}
                        _ => *x += 1,
                    }
                }
            }
            *octopi.get_mut(&coord).unwrap() = 0;
        }
    }

    pops
}

fn part_one(mut octopi: HashMap<(isize, isize), isize>) {
    let mut pops = 0;
    for i in 0..100 {
        pops += play_one_round(&mut octopi)
    }
    println!("{:?}", pops);
}

fn part_two(mut octopi: HashMap<(isize, isize), isize>) {
    for i in 0..10000 {
        if play_one_round(&mut octopi) == 100 {
            println!("{:?}", i + 1);
            break;
        }
    }
}

pub fn day_11() -> Result<()> {
    let inputs: Vec<Vec<isize>> = read_input("input/day_11.txt")?
        .map(|x| {
            x.chars()
                .map(|y| y.to_digit(10).unwrap() as isize)
                .collect()
        })
        .collect();

    let mut octopi: HashMap<(isize, isize), isize> = HashMap::new();

    for y in inputs.iter().enumerate() {
        for x in y.1.iter().enumerate() {
            octopi.insert((x.0 as isize, y.0 as isize), *x.1);
        }
    }

    part_one(octopi.clone());
    part_two(octopi.clone());

    Ok(())
}
