use std::ops::{AddAssign};
use regex::Regex;
use lazy_static::lazy_static;
use std::str::FromStr;

pub(crate) fn run() {
    let _input = "target area: x=20..30, y=-10..-5";
    // let _input = _get_input();

    let area: Area = _input.parse().unwrap();

    let best_launcher = find_best_launcher(&area);
    println!("best launcher: {:?}", best_launcher);
    println!("max height: {}", best_launcher.max_height());
    println!("valid initial velocities: {}", find_velocity_count(&area));
}

fn find_best_launcher(area: &Area) -> Launcher {
    let mut initial_y = 1i32;
    let mut working_launcher: Option<Launcher> = None;
    loop {
        if let Some(launcher) = try_get_launcher(area, initial_y) {
            working_launcher = Some(launcher);
        } else if initial_y > area.y_min.abs() * 5 {
            break;
        }
        initial_y += 1;
    }

    working_launcher.unwrap()
}

fn find_velocity_count(area: &Area) -> usize {
    let mut count = 0usize;
    for x in 0i32..area.x_max * 10 {
        for y in -area.y_min.abs() * 10..area.y_min.abs() * 10 {
            let launcher = Launcher { initial_x: x, initial_y: y };
            if launcher.reaches_area(area) {
                count += 1;
            }
        }
    }

    count
}

fn try_get_launcher(area: &Area, initial_y: i32) -> Option<Launcher> {
    for initial_x in 1..area.x_max {
        let launcher = Launcher { initial_x, initial_y };
        if launcher.reaches_area(area) {
            return Some(launcher);
        }
    }

    None
}

#[derive(Debug, PartialEq)]
struct Launcher {
    initial_x: i32,
    initial_y: i32,
}

struct Probe {
    speed: Coord,
    position: Coord,
}

impl Probe {
    pub fn step(&mut self) {
        self.position += self.speed.clone();
        self.speed.x += if self.speed.x > 0 {
            -1
        } else if self.speed.x < 0 {
            1
        } else {
            0
        };
        self.speed.y -= 1;
    }
}

impl Launcher {
    fn launch(&self) -> Probe {
        Probe {
            speed: Coord { x: self.initial_x, y: self.initial_y },
            position: Default::default(),
        }
    }
    pub fn max_height(&self) -> i32 {
        let mut probe = self.launch();
        let mut last_height = probe.position.y;
        loop {
            probe.step();
            if probe.position.y < last_height {
                break;
            }
            last_height = probe.position.y;
        }

        last_height
    }
    pub fn reaches_area(&self, area: &Area) -> bool {
        let mut probe = self.launch();
        let lowest_height = area.y_min;
        loop {
            if probe.position.y < lowest_height {
                return false;
            }
            if area.contains(&probe.position) {
                return true;
            }
            probe.step();
        }
    }
}

#[derive(Default, Debug, Clone)]
struct Coord {
    x: i32,
    y: i32,
}

impl AddAssign for Coord {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

struct Area {
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
}

impl Area {
    pub fn contains(&self, coord: &Coord) -> bool {
        coord.x <= self.x_max && coord.x >= self.x_min &&
            coord.y <= self.y_max && coord.y >= self.y_min
    }
}

impl FromStr for Area {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref REG: Regex = Regex::new(r"target area: x=(?P<x1>\-?\d+)..(?P<x2>\-?\d+), y=(?P<y1>\-?\d+)..(?P<y2>\-?\d+)$").unwrap();
        }
        let caps = REG.captures(s).unwrap();
        let x_min: i32 = caps["x1"].parse().unwrap();
        let x_max: i32 = caps["x2"].parse().unwrap();
        let y_min: i32 = caps["y1"].parse().unwrap();
        let y_max: i32 = caps["y2"].parse().unwrap();

        Ok(Self {
            x_min,
            x_max,
            y_min,
            y_max,
        })
    }
}


fn _get_input() -> &'static str {
    "target area: x=230..283, y=-107..-57"
}