use crate::common::read_input;
use anyhow::Result;

fn is_opening_delimeter(a: char) -> bool {
    a == '{' || a == '[' || a == '(' || a == '<'
}

fn is_closing_delimeter(a: char) -> bool {
    a == '}' || a == ']' || a == ')' || a == '>'
}

fn opening_delimeter(a: char) -> Result<char> {
    Ok(match a {
        '}' => '{',
        ']' => '[',
        ')' => '(',
        '>' => '<',
        _ => anyhow::bail!("Unexpected char"),
    })
}

fn part_one_score_string(input: &String) -> u32 {
    let mut openers = vec![];
    let mut error_char = '_';
    for a in input.chars() {
        if is_opening_delimeter(a) {
            openers.push(a);
        }

        if is_closing_delimeter(a) {
            if !opening_delimeter(a)
                .map(|x| x == *openers.last().unwrap())
                .unwrap_or(false)
            {
                error_char = a;
                break;
            } else {
                openers.pop();
            }
        }
    }

    match error_char {
        '}' => 1197,
        ']' => 57,
        ')' => 3,
        '>' => 25137,
        _ => 0,
    }
}

fn part_two_score_string(input: &String) -> u64 {
    let mut openers = vec![];
    let mut error_char = '_';
    for a in input.chars() {
        if is_opening_delimeter(a) {
            openers.push(a);
        }

        if is_closing_delimeter(a) {
            if !opening_delimeter(a)
                .map(|x| x == *openers.last().unwrap())
                .unwrap_or(false)
            {
                error_char = a;
                break;
            } else {
                openers.pop();
            }
        }
    }

    match error_char {
        '_' => {
            let mut score: u64 = 0;
            for i in openers.iter().rev() {
                score *= 5;
                score += match i {
                    '{' => 3,
                    '[' => 2,
                    '(' => 1,
                    '<' => 4,
                    _ => unimplemented!(),
                };
            }

            score
        }
        _ => 0,
    }
}

fn part_one(inputs: &Vec<String>) {
    let result: u32 = inputs.iter().map(|x| part_one_score_string(x)).sum();
    println!("{:?}", result);
}

fn part_two(inputs: &Vec<String>) {
    let mut scores: Vec<u64> = inputs
        .iter()
        .map(|x| part_two_score_string(x))
        .filter(|x| *x != 0)
        .collect();

    scores.sort();

    let result = scores[scores.len() / 2];

    println!("{:?}", result);
}

pub fn day_10() -> Result<()> {
    let inputs: Vec<String> = read_input("input/day_10.txt")?.collect();
    part_one(&inputs);
    part_two(&inputs);

    Ok(())
}
