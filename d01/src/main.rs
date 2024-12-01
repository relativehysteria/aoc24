use std::collections::HashMap;

fn main() {
    let input = &include_str!("../input");

    let (mut a, mut b): (Vec<i32>, Vec<i32>) = input.lines()
        .filter_map(|line| line.split_once("   "))
        .filter_map(|(a, b)|
            Some((a.parse::<i32>().ok()?, b.parse::<i32>().ok()?)))
        .unzip();

    a.sort_unstable();
    b.sort_unstable();

    println!("{} {}", part1(&a, &b), part2(&a, &b));
}

fn part1(a: &[i32], b: &[i32]) -> i32 {
    a.iter().zip(b.iter())
        .map(|(a, b)| (a - b).abs())
        .sum::<i32>()
}

fn part2(a: &[i32], b: &[i32]) -> i32 {
    let mut hist: HashMap<i32, i32> = HashMap::with_capacity(b.len());
    for &val in b { *hist.entry(val).or_insert(0) += 1 };
    a.iter().map(|n| n * hist.get(n).unwrap_or(&0)).sum::<i32>()
}
