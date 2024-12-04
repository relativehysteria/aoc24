// I wanna try using a hashmap instead of an array for once <3

use std::collections::{HashMap, HashSet};

fn main() {
    let grid = parse_grid(include_str!("../input"));
    println!("{} {}", part1(&grid), part2(&grid));
}


type X = i32;
type Y = i32;
type Grid = HashMap<(X, Y), char>;

fn parse_grid(input: &str) -> Grid {
    input.lines()
        .enumerate()
        .flat_map(|(y, row)| {
            row.chars()
                .enumerate()
                .map(move |(x, chr)| ((x as X, y as Y), chr))
        })
        .collect()
}

fn part1(grid: &Grid) -> usize {
    let needle = "XMAS";
    let deltas: Vec<(X, Y)> = (-1..=1)
        .flat_map(|y| (-1..=1).map(move |x| (y, x)))
        .filter(|&(y, x)| !(y == 0 && x == 0))
        .collect();

    let matches_needle = |x: X, y: Y, dx: X, dy: Y| -> bool {
        (0..needle.len())
            .filter_map(|i| grid.get(&(x + dx * i as X, y + dy * i as Y)))
            .collect::<String>() == needle
    };

    grid.iter()
        .filter(|&(_, &c)| c == 'X')
        .flat_map(|(&(x, y), _)| deltas.iter()
            .filter(move |&(dx, dy)| matches_needle(x, y, *dx, *dy)))
        .count()
}

fn part2(grid: &Grid) -> usize {
    let needles = HashSet::from(["MS", "SM"]);
    grid.iter()
        .filter(|((_, _), c)| **c == 'A')
        .filter(|((x, y), _)| {
            // Left-right; top-left > bottom-right
            let mut lr = String::with_capacity(2);
            lr.push(*grid.get(&(x-1, y-1)).unwrap_or(&' '));
            lr.push(*grid.get(&(x+1, y+1)).unwrap_or(&' '));

            // Right-left; top-right > bottom-left
            let mut rl = String::with_capacity(2);
            rl.push(*grid.get(&(x-1, y+1)).unwrap_or(&' '));
            rl.push(*grid.get(&(x+1, y-1)).unwrap_or(&' '));

            HashSet::from([lr.as_str(), rl.as_str()]).is_subset(&needles)
        }).count()
}
