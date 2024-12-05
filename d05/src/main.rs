type Rules = [[bool; 100]; 100];

fn main() {
    let mut input = include_str!("../input").split("\n\n");

    let rule_template = input.next().unwrap().lines()
        .map(|line| line.split("|"))
        .filter_map(|mut sp| {
            let a = sp.next()?.parse::<usize>().ok()?;
            let b = sp.next()?.parse::<usize>().ok()?;
            Some((a, b))
        });

    let mut updates: Vec<Vec<_>> = input.next().unwrap().lines()
        .map(|line| {
            line.split(',')
                .filter_map(|n| n.parse::<usize>().ok())
                .collect()
        }).collect();

    let mut rules = [[false; 100]; 100];
    rule_template.for_each(|(a, b)| rules[a][b] = true);

    println!("{} {}",
        part1(&rules, &updates),
        part2(&rules, &mut updates));
}

fn part1(rules: &Rules, updates: &[Vec<usize>]) -> usize {
    let sort = |a: usize, b: usize| rules[a][b];

    updates.iter()
        .filter_map(|update| {
            update.windows(2).all(|w| sort(w[0], w[1]))
                .then(|| update[update.len() / 2])
        })
        .sum()
}

fn part2(rules: &Rules, updates: &mut [Vec<usize>]) -> usize {
    let sort = |a: &usize, b: &usize| rules[*a][*b];

    updates.iter_mut()
        .filter_map(|u| {
            (!update.windows(2).all(|w| sort(&w[0], &w[1])))
                .then(|| update)
        })
        .map(|incorrect| {
            incorrect.sort_by(|a, b| sort(a, b).cmp(&false).reverse());
            incorrect
        })
        .map(|correct| correct[correct.len() / 2])
        .sum()
}
