use std::collections::HashMap;

type Antenna = char;
type X = i32;
type Y = i32;
type Position = (X, Y);

fn main() {
    let input = include_str!("../test");

    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();

    let mut grid = input.lines()
        .flat_map(|l| l.as_bytes())
        .copied()
        .collect::<Vec<u8>>();

    let mut antennas: HashMap<Antenna, Vec<Position>> = HashMap::new();

    for (y, line) in input.lines().enumerate() {
        for (x, chr) in line.chars().enumerate().filter(|&(_, c)| c != '.') {
            antennas.entry(chr).or_insert(vec![]).push((x as X, y as Y));
        }
    }

    let mut p1 = 0;
    let mut p2 = 0;

    for positions in antennas.values() {
        for i in positions {
            for j in positions.iter().filter(|&p| p != i) {
                let x = j.0 - i.0;
                let y = j.1 - i.1;

                for n in 1.. {
                    let nx = i.0 + x * n;
                    let ny = i.1 + y * n;

                    let in_arena = nx < width as X && ny < height as Y;
                    let in_arena = in_arena && nx >= 0 && ny >= 0;
                    if !in_arena { break; }

                    let idx = ny as usize * width + nx as usize;

                    match (n, grid[idx]) {
                        (2, v) if v != 1 => {
                            p1 += 1;
                            grid[idx] = 1;
                        },
                        ________________ => {
                            p2 += 1;
                            grid[idx] = 2;
                        }
                    }
                }
            }
        }
    }
    println!("{p1} {p2}");
}
