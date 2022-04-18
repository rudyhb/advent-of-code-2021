use std::fmt::{Debug, Formatter};
use std::str::FromStr;

pub(crate) fn run() {
    let _input = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";
    let _input = _get_input();

    let mut grid: Grid = _input.parse().unwrap();
    let mut i = 0u32;
    println!("start:");
    grid.print();

    while !grid.has_simultaneous_flash() {
        i += 1;
        grid.step();
        if i % 10 == 0 {
            println!("after step {}:", i);
            grid.print();
        }
    }

    println!("first simultaneous flash: after step {}", i);
}

struct Octopus(u8);

impl Debug for Octopus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Octopus {
    pub fn increase(&mut self) {
        self.0 += 1;
    }
    pub fn resolve_flash(&mut self) -> bool {
        if self.0 > 9 {
            self.0 = 0;
            true
        } else {
            false
        }
    }
}

impl From<char> for Octopus {
    fn from(c: char) -> Self {
        Self(c as u8 - '0' as u8)
    }
}


struct Grid(Vec<Vec<Octopus>>, u64);

impl Grid {
    pub fn has_simultaneous_flash(&self) -> bool {
        self.0.iter().all(|line| line.iter().all(|oct| oct.0 == 0))
    }
    pub fn print(&self) {
        for j in 0..self.0.len() {
            println!("{:?}", self.0[j]);
        }
        println!("flashes: {}", self.1);
    }
    fn get_neighbors(i: usize, j: usize, len_i: usize, len_j: usize) -> impl Iterator<Item=(usize, usize)> {
        let min_i = if i == 0 {
            0
        } else {
            i - 1
        };
        let max_i = if i == len_i - 1 {
            i
        } else {
            i + 1
        };
        let min_j = if j == 0 {
            0
        } else {
            j - 1
        };
        let max_j = if j == len_j - 1 {
            j
        } else {
            j + 1
        };
        (min_i..=max_i)
            .flat_map(move |i0| (min_j..=max_j).map(move |j0| (i0, j0)))
            .filter(move |val| val != &(i, j))
    }
    fn step_octopus(&mut self, i: usize, j: usize) {
        self.0[j][i].increase();
        if self.0[j][i].0 == 10 {
            for neighbor in Self::get_neighbors(i, j, self.0[j].len(), self.0.len()) {
                Self::step_octopus(self, neighbor.0, neighbor.1);
            }
        }
    }
    pub fn step(&mut self) {
        for j in 0..self.0.len() {
            for i in 0..self.0[j].len() {
                Self::step_octopus(self, i, j);
            }
        }
        for j in 0..self.0.len() {
            for i in 0..self.0[j].len() {
                if self.0[j][i].resolve_flash() {
                    self.1 += 1;
                }
            }
        }
    }
}

impl FromStr for Grid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut grid: Vec<Vec<Octopus>> = vec![];
        for line in s.split('\n') {
            let mut row: Vec<Octopus> = vec![];
            for c in line.chars() {
                row.push(c.into());
            }
            grid.push(row);
        }
        Ok(Self(grid, 0))
    }
}

fn _get_input() -> &'static str {
    "2344671212
6611742681
5575575573
3167848536
1353827311
4416463266
2624761615
1786561263
3622643215
4143284653"
}