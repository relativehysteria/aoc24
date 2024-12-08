type EqOperand = u64;
type EqResult = u64;
type Equation = (EqResult, Vec<EqOperand>);

fn main() {
    let input = include_str!("../input").lines()
        .map(|line| line.split_once(": ").unwrap())
        .map(|(res, nums)| (res.parse::<EqResult>().unwrap(), nums))
        .map(|(res, nums)| (res, nums.split_ascii_whitespace()
                .map(|n| n.parse::<EqOperand>().unwrap())
                .collect::<Vec<EqOperand>>()))
        .collect::<Vec<Equation>>();

    println!("{} {}", part1(&input), part2(&input));
}

fn part1(equations: &[Equation]) -> EqResult {
    equations.iter()
        .map(|(target, ops)| (target, part1_perm(*target, ops)))
        .filter_map(|(target, res)| res.then_some(target))
        .sum()
}

fn part2(equations: &[Equation]) -> EqResult {
    equations.iter()
        .map(|(target, ops)| (target, part2_perm(*target, ops)))
        .filter_map(|(target, res)| res.then_some(target))
        .sum()
}

fn part1_perm(target: EqResult, operands: &[EqOperand]) -> bool {
    match operands {
        [] => unreachable!(),
        [last] => *last == target,
        [rest @ .., last] => {
            (target >= *last    && part1_perm(target - last, rest)) ||
            (target % last == 0 && part1_perm(target / last, rest))
        }
    }
}

fn part2_perm(target: EqResult, operands: &[EqOperand]) -> bool {
    match operands {
        [] => unreachable!(),
        [last] => *last == target,
        [rest @ .., last] => {
            let mask = 10_u64.pow(last.ilog10() as u32 + 1);

            (target % last == 0     && part2_perm(target / last, rest)) ||
            (target >= *last        && part2_perm(target - last, rest)) ||
            (target % mask == *last && part2_perm(target / mask, rest))
        }
    }
}
