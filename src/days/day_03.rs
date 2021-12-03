use crate::common::read_input;
use anyhow::Result;

fn part_one(chars: &[Vec<u32>]) -> Result<u32> {
    let mut gamma: Vec<u32> = vec![];

    // For each digit. Take all the first digits and return the most common
    for index in 0..chars[0].len() {
        gamma.push(find_most_common(chars, index));
    }

    let epsilon = gamma.iter().cloned().map(|x| if x == 0 { 1 } else { 0 });

    let base: u32 = 2;
    let gamma_num = gamma.iter().enumerate().fold(0, |acc, (i, x)| {
        acc + x * base.pow((gamma.len() - i - 1) as u32)
    });
    let epsilon_num = epsilon.enumerate().fold(0, |acc, (i, x)| {
        acc + x * base.pow((gamma.len() - i - 1) as u32)
    });

    Ok(gamma_num * epsilon_num)
}

fn part_two(chars: &[Vec<u32>]) -> Result<u32> {
    let mut ogr: Vec<Vec<u32>> = chars.iter().cloned().collect();
    for index in 0..ogr[0].len() {
        let most_common = find_most_common(&ogr, index);
        ogr = ogr
            .into_iter()
            .filter(|x| x[index] == most_common)
            .collect();
    }

    let mut csr: Vec<Vec<u32>> = chars.iter().cloned().collect();
    for index in 0..csr[0].len() {
        let least_common = find_least_common(&csr, index);
        csr = csr
            .into_iter()
            .filter(|x| x[index] == least_common)
            .collect();
        if csr.len() == 1 {
            break;
        }
    }

    let base: u32 = 2;
    let ogr = ogr[0].clone();
    let ogr_num = ogr.iter().enumerate().fold(0, |acc, (i, x)| {
        acc + x * base.pow((ogr.len() - i - 1) as u32)
    });
    let csr = csr[0].clone();
    let csr_num = csr.iter().enumerate().fold(0, |acc, (i, x)| {
        acc + x * base.pow((csr.len() - i - 1) as u32)
    });

    Ok(ogr_num * csr_num)
}

fn find_most_common(chars: &[Vec<u32>], index: usize) -> u32 {
    let (zeros, ones): (Vec<u32>, Vec<u32>) = chars
        .iter()
        .cloned()
        .map(|x| x[index])
        .partition(|&x| x == 0);

    if zeros.len() > ones.len() {
        0
    } else {
        1
    }
}

fn find_least_common(chars: &[Vec<u32>], index: usize) -> u32 {
    if find_most_common(chars, index) == 1 {
        0
    } else {
        1
    }
}

pub fn day_03() -> Result<()> {
    let inputs: Vec<Vec<u32>> = read_input("input/day_03.txt")?
        .map(|x| x.chars().map(|x| x.to_digit(10).unwrap()).collect())
        .collect();

    println!("{:?}", part_one(&inputs));
    println!("{:?}", part_two(&inputs));

    Ok(())
}
