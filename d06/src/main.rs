// I didn't have time for this one :(

use std::collections::HashSet;

fn main() {
    let input = include_str!("../input");
    println!("{} {}", part1(&input), part2(&input));
}

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
struct VisitedMap {
    data: Vec<u64>,
    h: u32,
    w: u32,
    d: u32,
}

impl VisitedMap {
    pub fn new(h: u32, w: u32, d: u32) -> Self {
        Self {
            data: vec![0; ((h*w*d) as usize).div_ceil(64)],
            h: h,
            w: w,
            d: d,
        }
    }

    fn visit(&mut self, i: u32, j: u32, d: u32) -> bool {
        let address: usize = (i * self.w * self.d + j * self.d + d) as usize;
        let idx: usize = address >> 6;
        let bit: usize = address & 0x3F;

        let set: bool = ((self.data[idx] >> bit) & 1) == 1;
        self.data[idx] |= 1 << bit;
        return set;
    }

    fn visited(&self, i: u32, j: u32, d: u32) -> bool {
        let address: usize = (i * self.w * self.d + j * self.d + d) as usize;
        let idx: usize = address >> 6;
        let bit: usize = address & 0x3F;
        ((self.data[idx] >> bit) & 1) == 1
    }

    fn count(&self) -> usize {
        self.data.iter().map(|v| v.count_ones() as usize).sum()
    }
}

#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
enum Dir {
    Up = 0,
    Right = 1,
    Down = 2,
    Left = 3,
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct State {
    dir: Dir,
    i: u32,
    j: u32,
    h: u32,
    w: u32,
}

impl State {
    fn evolve<F: Fn(u32, u32) -> bool>(&mut self, is_obstacle: F) -> bool {
        let (ni, nj): (i32, i32) = match self.dir {
            Dir::Up => (self.i as i32 - 1, self.j as i32),
            Dir::Right => (self.i as i32, self.j as i32 + 1),
            Dir::Down => (self.i as i32 + 1, self.j as i32),
            Dir::Left => (self.i as i32, self.j as i32 - 1),
        };

        if ni < 0 || ni >= self.h as i32 || nj < 0 || nj >= self.w as i32 {
            return false;
        }

        if is_obstacle(ni as u32, nj as u32) {
            self.dir = match self.dir {
                Dir::Up => Dir::Right,
                Dir::Right => Dir::Down,
                Dir::Down => Dir::Left,
                Dir::Left => Dir::Up,
            };
        } else {
            self.i = ni as u32;
            self.j = nj as u32;
        }

        true
    }

    fn new_visited(&self, d: u32) -> VisitedMap {
        VisitedMap::new(self.h, self.w, d)
    }
}

fn parse(input: &str) -> (State, VisitedMap) {
    let points: HashSet<(char, u32, u32)> = input
        .split("\n")
        .enumerate()
        .map(|(i, l)| {
            l.chars().enumerate().filter_map(move |(j, c)| match c {
                '#' | '^' => Some((c, i as u32, j as u32)),
                _ => None,
            })
        })
        .flatten()
        .collect();
    let h = input.split("\n").count() as u32;
    let w = input
        .split_once("\n")
        .and_then(|(l, _)| Some(l.chars().count()))
        .unwrap() as u32;

    let mut obstacles = VisitedMap::new(h, w, 1);
    points
        .iter()
        .filter(|(c, _, _)| *c == '#')
        .for_each(|(_, i, j)| {
            obstacles.visit(*i, *j, 0);
        });

    let start: (u32, u32) = points
        .iter()
        .filter(|(c, _, _)| *c == '^')
        .map(|(_, i, j)| (*i, *j))
        .next()
        .unwrap();

    (
        State {
            dir: Dir::Up,
            i: start.0,
            j: start.1,
            h: h,
            w: w,
        },
        obstacles,
    )
}

pub fn part1(input: &str) -> usize {
    let (mut state, obstacles) = parse(input);
    let mut visited = state.new_visited(1);
    loop {
        visited.visit(state.i, state.j, 0);

        if !state.evolve(|i, j| obstacles.visited(i, j, 0)) {
            break;
        }
    }

    visited.count()
}

pub fn part2(input: &str) -> usize {
    let (mut init_state, obstacles) = parse(input);
    let mut init_visited = init_state.new_visited(4);
    init_visited.visit(init_state.i, init_state.j, init_state.dir as u32);

    let mut count = 0;
    let mut added: HashSet<(u32, u32)> = HashSet::new();
    let mut state = init_state.clone();
    let mut visited = init_visited.clone();
    loop {
        // advance the outer route one step so we can take its location as the
        // start point
        if !init_state.evolve(|i, j| obstacles.visited(i, j, 0)) {
            break; // left the grid
        }

        if !added.contains(&(init_state.i, init_state.j)) {
            added.insert((init_state.i, init_state.j));
            loop {
                // last is the location of the obstacle added for this point in
                // original loop
                if !state.evolve(|i, j| obstacles.visited(i, j, 0) || (i, j) ==
                        (init_state.i, init_state.j)) {
                    break; // left the grid
                }
                if visited.visit(state.i, state.j, state.dir as u32) {
                    // it's a loop - so terminate
                    count += 1;
                    break;
                }
            }
        }

        init_visited.visit(init_state.i, init_state.j, init_state.dir as u32);

        // fork the route starting from this point
        state.clone_from(&init_state);
        visited.clone_from(&init_visited);
    }

    count - 1
}
