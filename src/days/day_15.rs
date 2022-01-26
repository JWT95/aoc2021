use crate::common::read_input;
use anyhow::Result;
use recap::Recap;
use serde::Deserialize;
use std::collections::{HashMap, HashSet};

type Grid = HashMap<Point, u32>;
type Path = (Point, u32);

const VALID_MOVES: [(i32, i32); 4] = [(0, 1), (1, 0), (-1, 0), (0, -1)];

#[derive(Debug, PartialEq, Hash, Eq, Clone)]
struct Point {
    x: i32,
    y: i32,
}

fn print_points(points: &Grid) {
    let mut board = vec![vec![0; 50]; 50];

    for point in points {
        board[point.0.y as usize][point.0.x as usize] = *point.1;
    }

    for line in board {
        println!("{:?}", line);
    }
}

fn current_path_cost(path: &Path) -> u32 {
    path.1
}

fn expected_path_cost(path: &Path, endpoint: &Point) -> u32 {
    current_path_cost(path) + (endpoint.x - path.0.x + endpoint.y - path.0.y) as u32
}

fn build_part_two(grid: &Grid, len: usize) -> Grid {
    let mut new_grid = HashMap::new();

    for x in 0..(len*5) {
        for y in 0..(len*5) {
            // Take distance from original. Add to original and loop at 10
            let x_dist = x / len;
            let y_dist = y / len;
            let mut value = grid
                .get(&Point {
                    x: (x - x_dist * len) as i32,
                    y: (y - y_dist * len) as i32,
                })
                .unwrap()
                + x_dist as u32
                + y_dist as u32;
            if value >= 10 {
                value = value - 9;
            }
            new_grid.insert(
                Point {
                    x: x as i32,
                    y: y as i32,
                },
                value,
            );
        }
    }
    //print_points(&new_grid);

    new_grid
}

fn find_best_path(path: Path, grid: &Grid, endpoint: &Point) -> Path {
    // Start at the current point
    // Do not go back on yourself
    let mut unexplored_points = HashMap:: new();
    unexplored_points.insert(path.0, path.1);
    let mut visited_points: HashSet<Point> = HashSet::new();

    loop {
        let point_to_explore =  unexplored_points.iter().map(|x| (x.0.clone(), x.1.clone())).min_by(|a, b| {
            expected_path_cost(a, endpoint)
                .partial_cmp(&expected_path_cost(&b, endpoint))
                .unwrap()
        }).unwrap();
        unexplored_points.remove(&point_to_explore.0);
        visited_points.insert(point_to_explore.0.clone());

        for step in VALID_MOVES {
            let next_point = Point {
                x: point_to_explore.0.x + step.0,
                y: point_to_explore.0.y + step.1,
            };
            if grid.contains_key(&next_point) {
                let new_path = (
                    next_point.clone(),
                    point_to_explore.1 + *grid.get(&next_point).unwrap(),
                );
                if visited_points.contains(&next_point) {
                    // Do nothing
                } else if next_point == *endpoint {
                    return new_path;
                } else {
                    if let Some(value) = unexplored_points.get(&new_path.0) {
                        if *value > new_path.1 {
                            unexplored_points.insert(new_path.0, new_path.1);
                        }
                    }
                    else {
                        unexplored_points.insert(new_path.0, new_path.1);
                    }
                }
            }
        }
    }
}

fn part_one(grid: &Grid) {
    println!(
        "{:?}",
        current_path_cost(&find_best_path(
            (Point { x: 0, y: 0 }, 0),
            grid,
            &Point { x: 99, y: 99 },
        ))
    );
}

fn part_two(grid: &Grid) {
    let new_grid = build_part_two(grid, 100);

    println!(
        "{:?}",
        current_path_cost(&find_best_path(
            (Point { x: 0, y: 0 }, 0),
            &new_grid,
            &Point { x: 499, y: 499 },
        ))
    );
}

pub fn day_15() -> Result<()> {
    let input: Grid = read_input("input/day_15.txt")?
        .enumerate()
        .map(|(i, x)| {
            x.chars()
                .enumerate()
                .map(|(x, val)| (x, val.to_digit(10).unwrap()))
                .map(|(x, val)| {
                    (
                        Point {
                            x: x as i32,
                            y: i as i32,
                        },
                        val,
                    )
                })
                .collect::<Grid>()
        })
        .flatten()
        .collect();

    //part_one(&input);
    part_two(&input);

    Ok(())
}
