fn main() {
    let reports: Vec<Vec<i8>> = include_str!("../input").lines()
        .map(|line| line.split_whitespace()
            .filter_map(|n| n.parse().ok())
            .collect()
        ).collect();

    let p1 = reports.iter().map(|r| part1(&r) as usize).sum::<usize>();
    let p2 = reports.iter().map(|r| part2(&r) as usize).sum::<usize>();
    println!("{p1} {p2}");
}

fn part1(report: &[i8]) -> bool {
    // Determine whether the sequence is ascending
    let ascending = report[0] < report[1];
    let mut prev = report[0];

    for &n in report.iter().skip(1) {
        // Calculate the distance
        let diff = (prev - n).abs();

        // Make sure we're up to our conditions
        let distance = diff < 1 || diff > 3;
        let direction = (ascending && prev > n) || (!ascending && prev < n);
        if distance || direction { return false; }

        // Go next
        prev = n;
    }
    true
}

fn part2(report: &[i8]) -> bool {
    // If there's nothing unsafe, we're safe. Simple logic.
    if part1(report) { return true; }

    // To do this without bruteforcing, we'd have to retain the
    // ascending/descending boolean and the last level before the one that
    // breaks monotonicity. So that's that..
    (0..report.len())
        .any(|skip_idx| {
            let subset = report.iter().enumerate()
                .filter(|(idx, _)| *idx != skip_idx)
                .map(|(_, &n)| n)
                .collect::<Vec<i8>>();
            part1(&subset)
        })
}
