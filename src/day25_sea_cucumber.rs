use std::collections::HashMap;
use std::str::FromStr;

pub(crate) fn run() {
    let _input = "v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>";
    let _input = _get_input();

    let mut map = _input.parse().unwrap();

    move_all(&mut map);
}

fn move_all(map: &mut Map) {
    println!("initial state:");
    map.print();
    println!();
    while map.try_step() {
        if map.step % 10 == 0 {
            println!("After step {}:", map.step);
            map.print();
            println!();
        }
    }
    println!("After step {}:", map.step);
    map.print();
    println!();
    println!("took {} steps", map.step);
}

struct Map {
    step: u32,
    len_x: usize,
    len_y: usize,
    spaces: HashMap<Coord, Space>,
}

impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map: HashMap<Coord, Space> = Default::default();
        let mut len_y = 0usize;
        let mut len_x = 0usize;
        for line in s.split('\n') {
            let mut l_x = 0usize;
            for c in line.chars() {
                let contents = match c {
                    '.' => SpaceContents::Empty,
                    '>' => SpaceContents::SeaCucumber(SeaCucumber {
                        herd_type: HerdType::EastFacing
                    }),
                    'v' => SpaceContents::SeaCucumber(SeaCucumber {
                        herd_type: HerdType::SouthFacing
                    }),
                    _ => return Err(())
                };
                let coord = Coord { x: l_x, y: len_y };
                map.insert(coord, Space {
                    contents
                });
                l_x += 1;
            }
            len_x = l_x;
            len_y += 1;
        }

        Ok(Self {
            step: 0,
            len_x,
            len_y,
            spaces: map,
        })
    }
}

impl Map {
    pub(crate) fn print(&self) {
        for y in 0..self.len_y {
            println!("{}", (0..self.len_x).map(|x| match &self.spaces.get(&Coord { x, y }).unwrap().contents {
                SpaceContents::Empty => '.',
                SpaceContents::SeaCucumber(c) => match c.herd_type {
                    HerdType::EastFacing => '>',
                    HerdType::SouthFacing => 'v'
                }
            }).collect::<String>());
        }
    }
    pub(crate) fn try_step(&mut self) -> bool {
        let east_moved = self.try_step_herd(HerdType::EastFacing);
        let south_moved = self.try_step_herd(HerdType::SouthFacing);
        if south_moved || east_moved {
            self.step += 1;
            true
        } else {
            false
        }
    }
    #[inline]
    fn get_destination(&self, coord: &Coord, herd_type: &HerdType) -> Coord {
        let mut coord = coord.clone();
        if *herd_type == HerdType::EastFacing {
            coord.x = (coord.x + 1) % self.len_x;
        } else {
            coord.y = (coord.y + 1) % self.len_y;
        }
        coord
    }
    fn space_is_empty(&self, coord: &Coord) -> bool {
        if let SpaceContents::Empty = self.spaces.get(coord).unwrap().contents {
            true
        } else {
            false
        }
    }
    fn move_sea_cucumber(&mut self, from: &Coord, to: &Coord) {
        let mut temp = std::mem::take(self.spaces.get_mut(from).unwrap());
        std::mem::swap(self.spaces.get_mut(to).unwrap(), &mut temp);
        std::mem::swap(self.spaces.get_mut(from).unwrap(), &mut temp);
    }
    fn try_step_herd(&mut self, herd_type: HerdType) -> bool {
        let eligible: Vec<_> = self.spaces.iter().filter_map(|(coord, space)| {
            if let SpaceContents::SeaCucumber(sea_cucumber) = &space.contents {
                if herd_type == sea_cucumber.herd_type {
                    let dest = self.get_destination(coord, &herd_type);
                    if self.space_is_empty(&dest) {
                        return Some((coord.clone(), dest));
                    }
                }
            }
            None
        }).collect();
        let return_val = !eligible.is_empty();
        for (ref from_coord, ref to_coord) in eligible {
            self.move_sea_cucumber(from_coord, to_coord);
        }

        return_val
    }
}

#[derive(Eq, PartialEq, Hash, Clone, Default)]
struct Coord {
    x: usize,
    y: usize,
}

#[derive(Default)]
struct Space {
    contents: SpaceContents,
}

enum SpaceContents {
    Empty,
    SeaCucumber(SeaCucumber),
}

impl Default for SpaceContents {
    fn default() -> Self {
        Self::Empty
    }
}

struct SeaCucumber {
    herd_type: HerdType,
}

#[derive(PartialEq)]
enum HerdType {
    EastFacing,
    SouthFacing,
}

fn _get_input() -> &'static str {
    "....v.>v..v.vvvv...>..v....vv.>..>v>...v>.v>vv.v.>v.vv..>>>v>vv.>.>...>>v..v.vv..v>.v..>.>.>..>....>....>v.vv>>v.>v>.>.>...v...>..vv>..v...
vv>v..>..v......>..>>>.vv.....v....v>v.v.v.......>.v>...>.v..>>>.>>vvv.........v.>v....>>v.>>..>v.>.>.v.>.vv>>...v>...vv>.v>v...>>.>.>.>.v.
v.>v.vv...v.>....>.v....>.v.>v..>>..>>..>..v>..v.>.v...v.vv>v.v.v.vv..v>..v.....>.v>.v>v>v>.v..v..v..v>>.v..>....>..>.v.>v>v.vv..v..>v.....
.v>>vv...v.>....>>....vv>.>..>..>>.>v.v..vv>..v.>...>.>v.>v...>>v.v..v..>vv...>>vvv..>>v>>...vv>>...v.v>vvv..v>.....v.v.>.v>..>v..v..vv..>>
>.....>>>........>..vv>>>>>vv>v..v>>....v>>..>v........v.>v>v...vvv>v..vv..>....v>...v.v.v......>>..v.>...>..v>vvvv..>>..>.>.>.v.>vv>.vvv..
...>vv>>>>..v..>>>>>v>>..>>v.>>..>.v>v...vv.....vv.v>....vv..>....>>.vv.>..>.v>........>.>.v.....>>.v..vv.>.>vv.....v.v.....>v..vv.>...v>v.
...v>>.vv>v.....v>>..vv.v.v..v.>v.v.vv.vv>vvvv..vv>...vv>v.v.v.>.>>.v>>...vv..vv>..v>...v>.>.....v.v>.>v>>v.vv.v..>..>..v>..>v>..v>.>>>>..v
...>vv.v.vvv..>.>..vv..>>>>v.vvv>.>.>>v..v....>..v..>>>v>v.v>>v.>...v.v>>.>>.v>v>v.vvv>.v>...vv.....vvv.v>......v.vv>v..>v.v...>>>vv.>>.>..
>.vv>>v>.v>..>.vv>>v>..v..vv..>.....>vv.v..v..>>>>.......v>v.....v.>.>..>v..v.>>...vv.>..>.>..v....v.>>.vv...v..v....>v.vv.vv...>>>v>v>v>.>
.v..>.>.>v...>.v..v..vv.>>>.v....>>.>...>>>.>...>.v.vv.........v......>vv>.vvvvvv>v.v>>..>vv>..v>v>.v...>>....v.....>.v>.v...v>>>..v>v...>v
>.>....>v>v>.v>>...>>v.v>vvv.>..v..v.>vvvv..vv.>>.>>v..>>.v.>v..>.vv.v>.v>>.>...>.>vv.v>v...>.>v>>..>>.v..>.v...>..v>.v.>.v.vv......vvv...>
..v...v..>..v>>.vv.>...>..v>...v.v>...v..vvvv...vv.vv...vv..v>.>v.v>>>..v>..>>...>>.vv.v>.vvv....>>>..v>vv.>.v.v.v...vv>>.v.>>.v>>...v..>..
.............>..v.....v....vv.>......v.>>>v>v.>.vv.>.>>.>>..vv..vv.>v.>v.v.....>...>.......>vvv..>v>>.....>.>.>v>vv.v...>v..>>>v...v....>v.
....>.v>>>.>v.>..vv.v..v>v>...>>>.vv>..v>.>>.>..>.>......v..v...>v...vvvv..>...v>.v.>.>>>v>>.v>..>v.>vv.>v>v.v.>.v.vv>.>>..>...>>vv...v>v>>
vv>>.v>.>>.>>.v>.>vv.v>...v.vv.vvv..>v>>vv......>...vv>>..>v.v>v.v>v.>.v..>..v..vv>..v>vv>>.v>>v>>...v.>.>..v.vv.>>....v.v.>v.>...>..v>....
...>v.v.>.v>...v.>>>..v>>.>....v>.v...>..v>...v...........>vvv>>>...>>>v>.vv>>vv>v>vvv.v>..v>.>vv..v.v>..>.>>v.v..v..v...>>...>.....>vv...>
>..v>...vv..>>.>..v...v..>..>>......>>v...>>>.vv..v.v.v>v>.v.>vv.v>..>.>...>..>vv.>..>.>...>.>v.v.v>>v.....vv>v..>....>.vvvv.vvv>v.vvv>.v..
.>......vv>....v>>>..>...v.vv..v>>v.v..>vv>...vv>v>>.>v.v.v....>vv..v..>..>..>>...v.v.v.>>..>.>..>.v.>>..v>....>.v.>...>..v.>>>...v>>v....v
...v>>..>v.....>v..v>..>.v>..........v.>v>.v>v..v........vv>.v.v.....vv...>.>v.v>....>...v...vv......>vv....>.v>v.....v..>.....>>........>.
>>...v.>.....vv...>v>.v..>v..vv>vv.>.>..>.v>.>.v..v.>>...>...>>.....>>>.vvv.v>v>>vv.v>..v>.>>vv..>vv>v...>...>v...vv..v.>>.>v>>......>...v.
.>>.v.>.v.v>.v>>....v.....>v.v...v>v..>v.......v...>.>.>.>>.vv..>>v...v.>.v.>>v.v.....v..>>.>>>v>..>.>v>>vv>v>...>...>.v>v.>.v....v>..v>.vv
...vv>.vv>vv>.>v...>v....>>>>vv.>.>>..>.v>....>v.v>...vv>....>>.>.>v...>>v.>.v....v..>>..>.>v>v.v>..>.>.>>>>v>..v..v.>v>.>.vv>>v..>vv.v..>v
.....v>>>>..v>...v....>>.>.v..>..v..>>..v.v>..v.>.v>.vv...........>>.vv>.v>>.>...>.>v.>>>v..>..vv..>v.>....>v.v>.vv.>.....>>v>>>>>>>v..v>>>
.>..>.v.>.v..v..>>v>>.>>v..>>..>.v..>.>>...>..>v...>.v>vv>.vv>>v...>>.vv...........v.>>vv>..>.>>...v>.....>>.>>.v>v>vv...>..>....>.vvv.v>v>
.....vvvv....>.>>v..v.>.....v.>>>...>v>.vv..>v>>>v>.>v..vvv.v>v>..>..v>>.>vv.>.>vv>..vv...>v.>v..v>v.>...v........v>v.v.>..>...vv......v.vv
v.vvvvv.v>>....>v>..>>.>vvv>.>v.v.>..>v.>>vv....>..>.>..>.vvv.vv.v>..v..>v>v>>>...v>.vv>..>.v.>>>..v...vv...v>v>v..v.>..>..>>.......v..>v.>
>.v.>v>.vvv>.>vv.>v.>v..>v.vvvv.>v>..>>.vv.v>...vvv..vv..v>v..vv...vv..>.....>.>v..>v..>vv.v>v.>>>...>.v>........>>v>v.>v...>v.>..>.vv.vvv.
>>...v..v....>.>...vv>>v>.>.vv>>..v..v>.v..>....v>>vv...vvv.v...vvv>>>.vvv>...v>vv>v>..>.>...v>vvv>...>v.>..v>>..v....v>>v.....>..v>.v.>>..
vvvv.v>>vvv.>v..v.>..v..v>>.vv>.vv>>..v>>>vv.........>v....v>v>.>>>v>vv>..>.v>v...>>>vv.vv>>..>...v>>>....v.vv...>v..>>..v.v>>v..>>.>.>..v.
..>vv.>.>v>>>>>.v.....vv.vv>..v>.v.v>...>....v.>.v>.v>..>..v...v..>v>>.>>..vv..>v..v>v>.v..v.>.>.v>.v.v...v..>>.vv.v...>vv..>>vv..v.v>>>v.>
...>v...>.>v..v.v...v>v.>...>>v.vv.>v>v...>..vv..>.>>>.>.v>.v>.....>.vv....v..v.v...>.>.>>>>.>..>...v.>vv>.vvv....vv..>v..v>.....>...vv..v>
v>.>>.....>...v.>..v.v.>v.>....v.v.>>>>..vv>vv...>.v>v.v.>.vv>vv>v.v>.v>..>...v>.>vvv...>..>>.v.>.v..>..>.>>>.v>.v>.v.>>v.>>..>.>.v>>>>...v
>.>v..>v>>...v...>...v>v....v..v.>>.>..>.v.>.>.>v.v.>.v...>vvv..v.>..vv...>>vvv...vv....v...v>v>v.vv>vv>.vv.>v.v..vv..>>.>.>>.>vv..v.vv..vv
>v.v>...>.....v.>vv.>.vv.v..vv>.vv.v...>>..>>>.v>>v>.v>v>...v>vvv.>..>v.>.v..v...>.......v>>.vv..>v>.>.........>>....>>.>...vv...v..>..>.>.
v>vvvv...>...>......>...>>.>v.>>vvv.>.v>.v>..v>..v.v.v..v..v>.v.....v..v>...v>>vvv>..>..v.vvv>v>....vv..>...>..v>.>..>>.>..v...>.v.vv...>..
v..>>v..>.v.vvv..>....v..v>.>>.v>>..v.vvv.>>>>v..>.....>....v>v>vv...>...v.>..v.vvvv>.vv.>.>>>..>...>v>>.vv.v>v.>>.>...v>..v.>.v>.v>>.>..>.
......v>v>..>v.v..vv>>v..v.....vv.>v.>>>..>vv.v...vv..>.>v.v>>..>.v>>.v>>.vv.v>...v...v>>>vv>....v..vvv.v>vv>.v>.>v.>>..>..v..>>..vvv...>v.
>......v.>>.v>v...>...>..vv.>>>>>v..vv>>...>>v.v...v...>.vv>.vv.>....>.....v.>.>.>v>.v......v>..>>...>>>.vvv..vv...>...v.>.v>>>.>>.>..v..v.
...>..>...>...v.vv>.>>v>v>v..v...v...v.>.>.>..v..>>>....vv>.vv.>.vvv>>.v.v..v>..v..v.>...vv.....>vv.>>v..>v>v>.>>.>>.>.v>v.>>>>>vvv..>...v.
v...>>...vv>v...v>>...vv.>v..>>......>.>>..>....>v>.>vv>.>vv>...v..>.v.v....v>.....>.v.v.v..>v......v...vv.v..>vv>vv..>vv.v>>v.v>.>...v..v>
>>>>>..v>v..vv...>..>....>vv....>.vv.v>>v....>v.....>>>.>>>.>..vvv...>.>vv>vv.>v.v.......v.>.v.v..vv..v.>>vvv.>.>>....>.>.>.v.vv......v....
..v..vvvv.>.v>>.>...v>..>...>...>.v.>>v...>>v>..vv...>>..v>>.v>v..v.v.>....>..>..v.v.v>.>v>.....v>.>>>..>v.vvvv.v.v.........>...>v.>>.>.vvv
..v.>....vv......v...v>>>..>>..>.v..>.>...>>v.v....>>..>vv..>.>v.....v..>....v.>>v.>>...>..>.....>>.>...v.>vv>..vv>vv.>>.v.>.....>>>v...v.>
v......>v.v.>.>>v...vv>....vv...v>vvv...>v.v>>..v..>.>..>.>>...>..vv.v>.>..v.......v>v....>......v.>..>.v>vv.v>>.v>vvv.>...>v.v>.>>vv>>...v
v.v>..>.v.v..>vv.>.>>..v.....>..>.>v.>v.v.v.v.....>>>.vv.v.v.>.v..vv.v.>..>.v>v..>..>v>>.....v.v...>>.v.>v.>>v.v.v.v>.v.>>.vv>>.v....>.vv..
...v....v....v>v>...>.....v.vv.>v..>vv....>.vv.>>.v.v..>..v.>>>.v>v.>.vv>>.>vv>....v>v>.v...>>>....>.v>.v.v>>.>.v>.>>.v>>.....>.vvv.v.vv..v
v>....v.v>v..>..>...>.....>.v..v.>>>....v>>vv>.>..v.......>..>v>vv.vvv.v.v.vv.>...v..vv.....vvv.>>>.>v.........v.>.>.....>.v>>.v...>.v>.>vv
v.>v.v..>.v..vv..>>..v....vv...>.....>....v.v>>.>.>..vv.v>>>....v.>.>.vv...>>.v...>......v..v.v.>v>..>....>.>v..>v>.v.>.v..>...v.>v.>....>.
.....v.v>>>>>>v....>v.....v....>v.>>.>.>........>>..>v.>.v..vv>v>.v.vvv>....>...v.>>>>....>.>>>....>>v.>v>>.>...>>>.v>..vv...v>>..>..>v....
..>vv.>>.>>..vvv.>v>v>v.>..>>v...v>....v>.>...>v>.....>...>>..>v.vv.vv>..>v..>>>>.>>.........>.>...vv>.>>.v...>>..v...v>.v.vv>..>>v.>v..v.>
vvv....>...>>.vv>v>v.v......>..>.......>.>v..vv.>>>vvvv.v...vv>..>.v>.>....>....>.v.>.v.v>v>v..>..>..vvvv...>>.vvv>..vv>.v>v>>>>.....vv....
.>>.vv>..>v.....v....>..v>.v.v>..vv>v.v......>>>..>.>v>v>.v..>...v.>>v.>...>..>v......v>>.....>>v.>>v.v...>>.>>..>>>>.>....>..vv..>..vv.vv.
.v>...>>..>..>.>..vv.v..>vvvv>.>...>..v..>vv........v>.....v.>..>..>.>v...>.v>vv.v>>...>>>v>..>>..>v.vvv....v>v..v.>.v....v>>...>....v.v>.v
v>....>vvv..>.v..>..>..v.v.v>v>v>vv.>>.>....>v.....>v...v.>...v.>..v>.vv>..v>>.>vv....>v..vvv.v.vvv.>..>..vvvv..>...>.vv.v.>v..v>v..v.>>...
..>>>v>>.>>.>>......>.>....vv.v...>v>vv>.>.v>v.>v>>>v.>.v..v>....>.v...>..>.v.....v..>.>...v...vv.>>.v.>>...>.vv>...v...>.v>>>v.>vv>>v.>..>
.v.v.>.>>>>..>v.vv>v>.vv..v....v....v.>..>>.v>..v>.v>.v.>.>..v.>..>.>v..>>.vvv>.v......>......>.......>...>v.vv>v.v.>vv..v.>.>.>..>>v.>...>
>.>vvv....>v.........>....>...>..>.v.v.v>>vv>v..>.>v..>....v.>.>>.>.v.>>...v>>..v>v....>v>..vv>>>..v.>..>vv.vv.v>...>.>vv...>vv.v>..>.>v..v
....v.>..v.>.>>>v>...>v.....v...>...vv>v.>v.vv>vv..v..>>v>v..>......vv.v>.v.>v.>..>.v..>.>..vv.>..v.>.>..vvv.v..v..v.>.v..v>v.>vv>.>...>>..
..>..>v....v>v>>.v.>v>.>.>>>>.>...v>>>.v...>.v>..vvv..>>v...v>.>>.>>v......v.>v>....>>..v.>...vv>.v.>v..v..>>>...v>>.>>.>..>>.>....>>>..v>>
>.vv>>.>.v...vv>>.>vv.>v....v...vv....>v..>>....>v....>>>v>>....>....>v>.>...>v.v..>...v.v.....>.>vvv.v...v..vv>.v.v..v.v>.>>v.>v.......>v>
v...>vv>......>..>v..vvv.>.vvv>..v..v>.>vvv.v>.>vv..vv....v>.>>>.>.>>v..v.>>vvv.......>..v>>vvv.v.v..>.....>>v.>.v>>>...v.v.>..>>>vv.vvv...
v..vvvv....v>...>>>......>>..>>.>.v.v>v.v>>vv.vv.vv...>.v..>>v.>v..>>....>v..>...v...v>.v......v>vv...>..>>>.>.v>>..vv.>v.v>.>>.>.>>.v.>>..
v>.>..>>>.>vvv.v>vv.v>...>.v>>.v...>>v.>>...>.>>.v.v>.v..v>.>>..v>v>>v.....v>.vv..>v.>.>.>...vv>>.>.....>>..v>>..vvv..v.>...v>>.v>.>..>>>..
v.>...v.v.vv>v..>..>>>vv.vv.>vv.....v....v.....>v.>..v>v.>>v.>>....v....>...v..v>v>.>..v>.>..>...>.v>>>.v.v.vv.vv>v...v>vv...vvv..v..>v...v
>.>.v.vv..v.vv>.>...v>.....vv.vv>..v>.vvv.>.v>......>vv.>>.>>.v..v>.....>.v.>v..v....>..>>>.>>>..>>...>.>..v.>v.v..vv.v...>v.>vv..v>>>.v>..
>vvvv.v..v..v.>..>v>.v>vvv.>>>.vv>v..>v>.v...vv.>.v.vvv.v>v>>vv.vv.>..vvv.>>>vv..>>v>.v.>v>.....>>.>.v...v.vv...>.>..>..vv..>>vv>>...>>.vv.
>.>.v>.>v..v.v.v>>.>v.vv>>>>.vvv.vv..v>.>v.v..>.vv..>>.>.>>vv.v..>..v.v..>.....v>.>..v..>.vvv.v....>>.vvv...>..>>..>.v..>vv..>v.v..>>>vvvv.
.....v...v>>vvv.vv>>>>>v>.vv.vv.vv...>>..v..>.>.>>vv>.>...v.....v.>>.vv.>....vvv..>.>v.>v.v>>>v.v..v>vv..>.>..>.>.v...>..vv>>.>..>.>.>v>vv>
..vv...vv.v.>..>.>.vv..>.>.v>v...v>.....v>.>v..>vvvv>...>>>...>>v....>.>.v..>v..>.vv>v..v>>v..v>..v>...>>v.>>v.>..vv.....vv.v..>..>v..v>.vv
>v>v.>>>..>>.v......v....vv.v..v.>.v.v.v.v..>..>>..>..v..vvvv.>>....v.>>...>.v>.>>.v>>.>v>>..vv.vv>>v>.>v..>...>..>..v..>.......v..v...>v.>
v>>..>>vv>>>>v>v...>v.v.v....v>>...>>>>..>.v..v>.>....>.>.vv>>vv.vvv.>...v...>.v.v..v.>.>..v>..>.v>.v>.>>v>v>vv...v.>>>..v>.v>.>...vv.v>...
.>.v...>..>.v..vv...>.vv.v>v..vv.v>.>v...>>>...v>.>.>..v>...v.v.vv.v.>>>vvv.vv.>.v>.>.>v>v>v.vv>...>.>.v>.>....>...>>.>vvv.v.v.....>..>v..v
>.>....>v..v...v..v>.v.>..v>>v.vvv>.v>.>v.v.>v>vv>..>..>>vv...v.v..v..>v..>.vv.>..>..>....v>v.>vv.>...>....>v...v>>>>v>...v>>vvv>>>v..>vvvv
v.v>vv..v.>>.v>>>.v..>..>.>>..>>>v.v>>v...v..>>.>.....>..v..>vv>........vv...vv>.vv.vvv..>..v>.v.v...vvv>>v..>.>..>v>>..>......>>vv...v...>
.>v>..v>.v>.v..vv>.v>....vv.....v.>v.v...v.vv>.>>.>.>v..>vv..v..v.>v>>v...v.>>>.>...v...>v.>>..>.>.v.>vv..vv>..vvv.v>.vv.vv>.>.vvv.>>v.>.v>
....v..v..>.>.>>.>.v.>......v.v.v>.>>...v.v...vv.>vv..>.vv..v...v....vv>.>>>v...v......>>>.>.>v.vv.vv..>.>.>.>..>>...v>....vv..v.>...v..v.v
.>>vv..vvvv.>....>>.vvvv...v.v>....v..v.>.>>.>v.>>v>v..>...>>....>>v..>.....>.>.>>v.>vv>.>.......>.v.v.v>v...>.>......>v.v......v.>v....v.>
>v..v>.>....v>>>>..>..>>v..>>.v.v.v.v.>.>...v..>v>>.v.v.v......>..>.>v>...vv>..v....vv.>.vvv...>>>v>v.v...vv>>.....>>vv.>..>>>v.>.v>.v.v>v.
.v.>>.v....v>v.......v..>.v...v.>.v.>vvv..>>..>.v.>>v>v.>.>v..>v>>..>>..>..v..>.vv..>...vv..........v..>.v.>..>vvv..v..v..v...vv..vv>>.v.v.
....v.>>..vv>..v>........v.>>.v>.v>...>..>.vvv>.v....>..vv.>..v.v.>.v>>.>..vv..vv..>v>>>..v..v...>v...v...>.v.vv.vvv...v.>.>..vvvv.v>>>....
.>.vv>.v.vv.>.v..>.>..>>....>.v>.>>v..vv>.v.....>v>>>>.>>.v>.....>...>v.>v.>vv...v..>vv..>.v>.>>>.>v...>.>.v...>>.>>>vvvvvvv.v....>...v>vvv
v>....>>..v..vv>>.>.v.v>v.........>...v.v>v>vvv>..>>.....v..>..>v..v>>.vvvv>v.v...>v...>..>v>v>vv....vv>.>..>>vv>.v>....>>.v>v......>>.v>v>
>.vvvv>....>...>>v..v...>.>v...>v..vv..>.>.v..>v>.vv>.>.>v..v>.v.vv..vvv...v.v.v.v.v.>.....vv>....v>>v..>>.>....>v>v.>v..>>......>.v>..vv>.
.vv>..v.>vvv>v.v.vvv>..>..>.>v..v.>>>.vv..v.>>.....v...v..>..v.>>....>>>>vv.>.v.>>>..v...v.>...>>...v>v..>v>.>.vvv.>>...vv...v..>.>>...>>.v
>>.>>v.>..>>.v.v..v>..>.v>>.>..v..>..>.v.>....>.>>>v>.>..v>vv.....>vv.>..vv...>.>.>v....>......vv..>..>.v.>>v.v..vvv.vv.v....>.....>vv..>.v
>...v>.>..v.v>>v>.v.>v>...v.vvv....v....v.v..>v.v.>v>vv........v>...>v>.>vv...v.>>v........v>.>.>>v.v...>.>.vv..>.v..>>..v..vv>v...>.vv>...
....v.v...>v>...>....vv.>v...>.>v>>>....v..>vv>>v.v...v...>..>..>>.>...>.>.vvv..>.>vv..>v>v>..vv..>.>.>>v>v..>v>v>.v..>>vvv>........>.v..vv
.....v.v>..>>...v>v.v........>.>.>...vv.v>>v....>>.>v.>v....v...v.v....vv...v.vv..vv.>v>>v....v.>.vv>v......>....>..v>>v.v.vv>..>.v..v>>v.v
v>v.v.v.v.>.v.v.>vvvvv.>v>.v.>v......vv...>>...>>>v.>v>>>>v>vv>>>vv...v>vv>.v>.v..>........>.>.v.v..>.>>.>.....v...vv..v....v.>.v.>v.>...>.
>..>>v.>.v...>v>v..>v..>..>.v>.>.....>...v.......>...>v..>.>>v.>..v.v.>.>>.>>v.v>.v..v.>>>v...vvvv.>v..v.v.vv>.>...v>v>>.v.>v...>v......>v>
>v..v.vv>v.....v.>v>v>>...>.v......v.....>.vv>...v.>.>..>>..v>.>vv.vv>>.>>>v.v>>v>vv..v..>vv..v>v>....>v..>v>..>v..v..vv.v..>>.....>.vv.v.v
>>v...vvv.>>.>.v.vv.>>>>........vv.....>..v>.vvv...>v....>>vv>.....v.>.>>.>v..vv..vv>...>v>>>vvv.>....>v>..>....v.>.>.vv>v.>...>>.......vv.
v>>.>>v>.v>vv...v.>.>v..>.....>..>.vv>>v.v>>..vv>>..>.>..>>vv..v.>..v.....vv....>v.....vv.v>...>v...>v>>.v>....>v..v>>>.>.vv..>v..v..v..v.>
.>v.vvv...>.>........v..>>..v..vv>..>..vv.v>.v>v>v>.v>>.v.>.vv..v.v.>>>>.........>v>v>.vvv.>.>v..>v.v.v...>.>>vvvv.>.v>v>vv.>>>vv>v>v.....v
>.v.>.v.v..>......>..v....>vvv..v>v.vv.>...v>.vv....>v.>>>v..v..v.>>>>v>.>.....v.>>..>.....>.>..v>.>>v..>...v.v>>v.>.>.>.>.v>.v...>v...v..>
v......vv.v......>v.>>v.v>vv>.v..>.v....>>.v...vvv...v>>vv>>..>..>>..v>>v.....v>.......vv.v....v>...v>....v.v>v.........>..>>..>.>>..v>v...
v.>.v..v..v>.>>...>.>.v.vv...v..>v..>vv>..vv>v..vv...>..v...v...v>>..>.v.v.....>v>>..v.v>..v>...v..>>vv.>.v.vv.>.v..v..>>>>v>......>..>.>vv
...>..>v.v.>..>.>>v..>v..vv....>v.>v.v>.>v.>...>.>v..>....v...>...vv.>>>.v>v>.v...>>..>>.>..>.vvv..v.>..v.v..>.>>>>v.....>..>v..>v..v>....>
.v>>.v.v.>>..v..>>v>..v>....v.v>v........v>..>v...>..vv..vvvv>...>>>.>v......v>vv.>..vvvvvv...vv..v..>v.v.v.vv.v.v>vv>.>>..>..v..>v>..v.>v>
>>>.>vv>..>....>.>..>..v.>v>>.>.v.....vv....>v.....>>>.v...>v>v>.vv.>.>...>..vvv.v.>...v>vv..v>..>.>..v>..>.v...>.>vv..>v>vv...>>..>.>vv..v
.vvvv.v>.>.>vv>>>v>.>.v>.>..>.>....>v>vv>..>..v>vvv>..>>..>v>.v>.v.v>...v>>.v.......>.vvv>.....>>v>v.>>v...>.vv.vv..>v>..vv>..v>v.>v>v...v>
>v.v.v.v>v.....>v>vv.v.v>..>.v>>v.v.v.>>.v...>..>..>.vv>.v>>..v>.vvv..>vv>v>>..vv.>.vv>..>>...v.>vv...v>>vv..v..>..vv.v....vv.>v.>vv.vvvv.v
.>..v..>>v>.>..vv....>>.>.....>.>..>>>>>>.v.v...v.v.>>vv.>.>vv.>>>........v>...v...>v.....v>.v>.v.v>v..v....vv>...vv...v.vv>.>.>..v.v.>..>v
v.v.>.v.>.v.v>v>>.....>...v....>vvv..>v>>>..vv>.>>..v>>>>.....v...>...vv..v>>>...v>..>..>..>.>...>>....v.>v>....>..>....vv...vv.>v>v>...>..
>.>..>..>..v>v.>>>..v>.v>.vv>..>..>v.>.vv...v>.....>>.>...v..>vvv>v......>v..v.>>......>..v>vv>.v...>.v>v.>..v.v.>v.....>v>v..>vv.>.>>>>>>v
>.v>.v.vvvv>vvv..>...>vvv.v>..>..>.v......>..v>>.>...>>..........>vv>>>.vv.....v.>.>.>v>vv...>v>>.....>..>..v...>>v..>...>>>.vv...v..vv.>>.
v...vvv.v.....v.v.v.v>>v>v.vv...v.v..>>v..>.>>.vv..>.v...v>.vv.>>..>...v.>>...>.v..v>v>v.>>v.v.>v..v...vv.>.v.v>>.v.>.....>.>....v>...v>vv.
..>..v.>.>.v>.v>v>>>>>v.v..>>...>.v..>v..v>v...vv.>>v.v.......v>>>v.v>v.>..>.v..>>>...v.>>...>vv....v>v>>.v.v.>..v>>v.v>vvv...vv..>.v>vv.>>
v>>>v>v>.vv.v..vv.v..>>.v.>>>..v.>v>>.....v>..vvv.>>..>.>>.>.v..v...>...>.>v..v>v.>.>.v..>>...>...>v.vv>.>>.>v>.vvv...v.>.........vv.>>.>.v
>.v..>v....v......>.>>v.>>vvvvvv>.v.v.v>>v..>vv>.v.v>>.v>v>....v...vv>v>>..v..v..>vv..>..vv>..vvv..v.>..>>v>v.>.>.v.>>.v.....vv.vv..vvv>v..
v..vv..>.vvv>>v.>.v..v>>.v...v.v>...v..>>>...>....vv>....>.>vvv.v>....>.v.v.vv.v..vv.>vv>..vv..v.v....v.>.>.v>..v.v..>>.v....vvvv>>>..>>..v
>.>.>...>..vv.......v.vv.........>vv..vv>..>..v..v.>v.v.v...>>>...v.vv>v...vv>>v>..>.v..v.>.v>>v>v..>v>>>.v..>>>........v...v>>...>..v.>..>
>..v>v.v...>v...v>>..>....vv>.>...>>.vvvvv>vvv.>..>...>v.v>v.>.>vvv...vvvv>>>>>>..vv.v.>>>v.>vv....vv..v>.v.>.vv......v.>.vv>..v.>>>...v>>>
.>>.v..>>.vvv.>.>..>..>>..>.v.vv..>>.v.>>>vv>>.v>v.>v>>.v.v...>...v..>>.vv.v...v..>...vv>>.v.v>v>.>.>v....>>v>.>....v.>.>.>.>v.vvv.v.vv>.v.
>.v>.>v.....>.>vvv..>.>v.>...v>>.>.v..v>.>..>.>.vv...>...vv.v>..>vv...>..>>.>.>..>v>.>>..>.>.>>.>>vv....>>.vv>>.v.v...>..>..v..>>vv.>>...vv
....>>.>v.v..v..vv>>v>>.>.>.vvvv.v..>.>>...>vvv>v..>.v..>.>>v.......v.v.>v....v...>.v.>.......v.>>>......>>v.v.>.v>.>..v>..>..>..vv..>...v.
>v>>..>...>vv..>vvv..vvv.>.>v...v...v>...>.>...v.v..>.v.>.vv..>.vvv.>.vv>.>>v>.v.vv>.>v>.v>v>v.>>.v>>v...v...>.>>.>v..vv.>v..>>v.>.>v..>>>>
vv....>...>v>..>...v..v.>v>.>>.>..v...v>.>>>v..>v.v>>v>>..>>v>..vv..>.>>>.v>.>...v>v.vv.>>vv..>v>.v.>.>vvv..v>..v>vv...>v.v..>......v.>v.>v
....v....>.>...v>>vv.>.>..>>..>..>>v>..vv.v.>....>.>..v>...v.>.v>...>v...>....v.>.>..>.>v..>.....v....>.v..>vv.>.>>vv>...v.vv..>...v>vv...>
>.>..vv.v>>>.v>vvv>v..vv....>........>.v.>>....>.v>>.vvv>.>v..>v.>>v....v>..v..v>vvv>.v>.vv..vv.>vv..v.>.v>>..>..>.....>..vv.v>>....v>.v.v.
v..>..>vv.v.v>.....vvv>..>v>v.....>.v>>>>>vvv>..>.>v.v>.vv.>.v.vv....>.v..v....v>...>>.>.>>.>..>..>...vv>v>v>.v...v.vv>.v>.....>.....>>...v
vv.>v.>v.v.>....v.>v>v>>.>...v.vv.>.vv>....v>>>.vv......>...>vvv..>>>.>.>......>v.vv>>.>...........>.v..>..>>v.vv..v.>>.v>v..v>>v>>v..>.>vv
.v..>v.>v...>..v...>>..>......v.vv.>>>..v>..v.>.>>..v.>.....vvv.>>....>>>>v>.v.vv.>...v>>....v.v.>.>.>.v..>...v.v>>>.v........>...>.vv.....
>>.v>...v.>.v..>..>vv.....>vvv..>..>>v.v...>>v.>vvvv>...vv...v..>>>.>.>.vvv..vv..vvv.>>.>.v..>v>>.>v...>vv..v.v.vvv.>.>>>.>v..>>.....v.>v>v
.>.......>.>>v>.>..>.>..>.vv.>v>..>v.>vv..>vvv>>..>.v...v...v..v.v>v>....>.v....vv>.>v>v.v...v.....>..>>>....v>v...v..>.....>..>......v>vv.
vv>..>v>.......v>>>v>>.>vv..>vvv...>>..>>vv...v>..v...v...v>vvvvvv...vv..>v.vvv..v.vv....>..vvv>>......v....vv....>.vv.>..>v.>..>v>..>.>...
.v>v..v.v...v>..>>v.vv...>.v.v...v>.v>.>>.......>.vv..v.v>>>>.v..vv>..>.vv>.>.v>>.>.v>.......>.v.v.v.>v...>>.>>v.vv>.v.>.>>>..vvv..>>>v>.v.
v>..vv>>vv..v...>v.vv.v.>..>.vv..>v.v..>vv.v...>.v>.>...v>v.v.>vv.......>>.vv>>.vv>v.vv>v>>.v>vv..>>..>.vv...>>.>.v.vv.>v.>>>....>>.v>>.>vv
>...v.>.v..>vv.vvvv.......>>...>.>v...v.v..>.v....>>.>.vv.>v>.>..>>>....v>..>.vv>..>...>v.>.>.vv>.v..>..>>v.....>.>.>........vv.v>..>.....v
..v>.>..v.>.v>>................v.v>v.....v>>>.....>...v..>.v>...>>>.v>..v.>..>>.v.>..>..vv.v>>>.v>.>..v.v.>v.>......>..>.>v.>.v>.>>>v...v.v
v>..vv...>.>.>..v.......>.>..>>v>v.v.v.v.v..v...v.vv>>.v>v>.......>>vv.>.v.>v.....v..v>v.....v..v.>...>.....>>.>v.>>...>>.v.>.v.>v>.>.v..v.
v.v>....v>v.v.vv.v.>..vv.....v>..vvv.>.vv.v>>..v.v.>.v...v..>..>.>>v..v.v..>.vv>>v>vv..vv..v>...>.>.vv.v..v.v...v.>.v.>.v.>>>v.v.v.>>v...vv
vv.vv.vv.>....v..vv>.>>.v.>v...>.v...v..>.>>v>>>>v...v.v.>.v>..>v.v>...>>...v......v.v>.v....>.v.>>v>v>.>>v.v>>..>>v.vv.>>v.>.v...>v.v.....
v.>..v.v..>vvv>>v>v>>.v>>v..>v>...>vvv..>v.>vv.>.>>..v.v....v.>....vvv>.>..>...>>..>>>.v.v.>v>vv>..v...v.>...v>vvv...v.>.....v..v.v..>v>...
>.>>v.>.v..>.....>..>>v...v>..>.>.....v....v.v.v.v.v.>.>v..>......>v.v..>>..>>....>>>vvvv...vv.v.vv..v..v.>.vv>vv.v..>vv.v>>vv..>.v.v>>..>v
...>>v>..v..v.v.>v.>..v.>.>..v..v...>>.>>..v.>.>vv..>.....vv.>....>v.>vv>..v>.v.v......v..v>.>>v.>v>...>...>...>>.>...v>vv>>..v...>>>v.v>.v
vv..v>>..v..v.>..v...>vv.v>.>>vv>v..v..v..>.v>vvv..v...>v>v>.>>..>.v>.v.>.v.v..vv...v..>v.......>.....>.v.v...vv>vv..vv..>.>.v>>v>>v...>>.v"
}