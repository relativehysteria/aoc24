use std::collections::HashMap;

fn main() {
    let input = include_str!("../input");

    let (mut a, mut b): (Vec<i32>, Vec<i32>) = input.lines()
        .filter_map(|line| line.split_once("   "))
        .map(|(a, b)| (a.parse::<i32>().unwrap(), b.parse::<i32>().unwrap()))
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
    let histogram = |arr: &[i32]| {
        arr.iter().fold(HashMap::new(), |mut histogram, &val| {
            *histogram.entry(val).or_insert(0) += 1;
            histogram
        })
    };

    let a_hist = histogram(a);
    let b_hist = histogram(b);

    a_hist.iter()
        .map(|(key, &val)| key * val * b_hist.get(key).unwrap_or(&0))
        .sum()
}
