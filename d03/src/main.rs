use regex::Regex;

fn main() {
    let input = include_str!("../input");

    let p1 = part1(input);
    let p2 = part2(input);

    println!("{p1} {p2}");
}

fn part1(input: &str) -> usize {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    re.captures_iter(input)
        .filter_map(|c|
            Some((c[1].parse::<usize>().ok()?, c[2].parse::<usize>().ok()?)))
        .map(|(x, y)| x * y)
        .sum()
}

fn part2(input: &str) -> usize {
    input.split("do()")
        .map(|line| line.split("don't()").next().unwrap())
        .map(|dos| part1(dos))
        .sum()
}
