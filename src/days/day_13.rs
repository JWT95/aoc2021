use crate::common::read_input;
use anyhow::Result;
use recap::Recap;
use serde::Deserialize;
use std::collections::HashSet;

#[derive(Debug, Deserialize, PartialEq, Recap, Hash, Eq, Clone)]
#[recap(regex = r#"(?P<x>\d+),(?P<y>\d+)"#)]
struct Point {
    x: isize,
    y: isize,
}

#[derive(Debug, Deserialize, PartialEq, Recap)]
#[recap(regex = r#"fold along (?P<direction>\S)=(?P<line>\d+)"#)]
struct Fold {
    direction: char,
    line: isize,
}

fn perform_fold(points: HashSet<Point>, fold: &Fold) -> HashSet<Point> {
    let mut new_points = HashSet::new();
    match fold.direction {
        'x' => {
            for point in points {
                let diff_from_fold = fold.line - point.x;
                if diff_from_fold == 0 {
                    continue;
                } else if diff_from_fold > 0 {
                    new_points.insert(point);
                } else {
                    new_points.insert(Point {
                        x: point.x + 2 * diff_from_fold,
                        y: point.y,
                    });
                }
            }
        }
        'y' => {
            for point in points {
                let diff_from_fold = fold.line - point.y;
                if diff_from_fold == 0 {
                    continue;
                } else if diff_from_fold > 0 {
                    new_points.insert(point);
                } else {
                    new_points.insert(Point {
                        x: point.x,
                        y: point.y + 2 * diff_from_fold,
                    });
                }
            }
        }
        _ => unimplemented!(),
    }

    new_points
}

fn print_points(points: HashSet<Point>) {
    let mut board = vec![vec!['.'; 40]; 40];

    for point in points {
        board[point.y as usize][point.x as usize] = '#';
    }

    for line in board {
        println!("{:?}", line);
    }
}

pub fn day_13() -> Result<()> {
    let input: Vec<String> = read_input("input/day_13.txt")?.collect();

    let points: HashSet<Point> = input
        .iter()
        .take(799)
        .map(|x| x.parse::<Point>().unwrap())
        .collect();

    let folds: Vec<Fold> = input[800..]
        .iter()
        .take(799)
        .map(|x| x.parse::<Fold>().unwrap())
        .collect();

    println!("{:?}", perform_fold(points.clone(), &folds[0]).len());

    let mut folded_points = points;
    for fold in folds {
        folded_points = perform_fold(folded_points, &fold);
    }

    print_points(folded_points);

    Ok(())
}
