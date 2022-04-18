use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use std::thread;

pub(crate) fn run() {
    let _input = "#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########";
    let _input = _get_input();

    let _input = _modify_input(_input);
    let _input = &_input;

    // let amphipods = get_amphipods(_input);
    // let mut burrow = Burrow::new();

    let amphipods = get_amphipods_v2(_input);
    let mut burrow = BurrowV2::new();

    burrow.init(amphipods.into_iter());

    let burrow = get_min_cost(burrow);
    println!();
    println!();
    burrow.print();
    println!();
    println!("minimum energy is {}", burrow.get_cost());
}

fn _modify_input(input: &str) -> String {
    let mut output = String::with_capacity(input.len() + 28);
    let mut lines = input.split('\n');
    output.push_str(lines.next().unwrap());
    output.push('\n');
    output.push_str(lines.next().unwrap());
    output.push('\n');
    output.push_str(lines.next().unwrap());
    output.push('\n');
    output.push_str("  #D#C#B#A#  ");
    output.push('\n');
    output.push_str("  #D#B#A#C#  ");
    for line in lines {
        output.push('\n');
        output.push_str(line);
    }

    output
}

fn get_min_cost<T: AmphipodBurrow>(burrow: T) -> T {
    const PARALLEL_THREADS: usize = 12;
    let mut states = HashSet::new();
    states.insert(burrow);
    let mut i = 0usize;
    while states.iter().any(|s| !s.is_complete()) {
        println!("step {}", i);
        i += 1;
        // for s in states.iter() {
        //     println!();
        //     s.print();
        // }
        // println!();
        // println!();

        let result: Arc<Mutex<HashSet<T>>> = Arc::new(Mutex::new(Default::default()));
        let mut threads = Vec::new();

        let mut states_arr = Vec::from_iter(states.into_iter());
        let chunk_size = states_arr.len() / PARALLEL_THREADS + 1;

        for _ in 0..PARALLEL_THREADS {
            let mut states_chunk = Vec::with_capacity(chunk_size);
            let mut i = 0;
            while let Some(s) = states_arr.pop() {
                states_chunk.push(s);
                i += 1;
                if i >= chunk_size {
                    break;
                }
            }
            if !states_chunk.is_empty() {
                let result = Arc::clone(&result);
                threads.push(
                    thread::spawn(move || {
                        let next_states: Vec<T> = Vec::from_iter(states_chunk.into_iter().flat_map(|s| s.get_next_possible_states()));
                        let mut result = result.lock().unwrap();
                        result.extend(next_states);
                    }));
            }
        }

        for t in threads {
            t.join().unwrap();
        }

        let lock = Arc::try_unwrap(result).expect("lock still has multiple owners");
        states = lock.into_inner().expect("mutex cannot be locked");
    }

    states.into_iter().min_by(|a, b| a.get_cost().cmp(&b.get_cost())).unwrap()
}

#[allow(unused)]
fn get_amphipods(s: &str) -> [Amphipod; 8] {
    const VALID_CHARS: [char; 4] = ['A', 'B', 'C', 'D'];
    let chars: Vec<char> = s.chars().filter(|c| VALID_CHARS.contains(c))
        .collect();
    [Amphipod::new(chars[0]),
        Amphipod::new(chars[1]),
        Amphipod::new(chars[2]),
        Amphipod::new(chars[3]),
        Amphipod::new(chars[4]),
        Amphipod::new(chars[5]),
        Amphipod::new(chars[6]),
        Amphipod::new(chars[7])]
}

#[allow(unused)]
fn get_amphipods_v2(s: &str) -> [Amphipod; 16] {
    const VALID_CHARS: [char; 4] = ['A', 'B', 'C', 'D'];
    let chars: Vec<char> = s.chars().filter(|c| VALID_CHARS.contains(c))
        .collect();
    [Amphipod::new(chars[0]),
        Amphipod::new(chars[1]),
        Amphipod::new(chars[2]),
        Amphipod::new(chars[3]),
        Amphipod::new(chars[4]),
        Amphipod::new(chars[5]),
        Amphipod::new(chars[6]),
        Amphipod::new(chars[7]),
        Amphipod::new(chars[8]),
        Amphipod::new(chars[9]),
        Amphipod::new(chars[10]),
        Amphipod::new(chars[11]),
        Amphipod::new(chars[12]),
        Amphipod::new(chars[13]),
        Amphipod::new(chars[14]),
        Amphipod::new(chars[15])
    ]
}

trait AmphipodBurrow<T = Self>: std::clone::Clone + core::hash::Hash + Eq + std::fmt::Debug + std::marker::Send + 'static {
    fn get_cost(&self) -> usize;
    fn get_space(&self, coord: &Coord) -> &Block;
    fn get_space_mut(&mut self, coord: &Coord) -> &mut Block;
    fn increase_cost(&mut self, amount: usize);
    const AMPHIPOD_ROW_COUNT: usize;
    fn new() -> T;
}

trait BurrowCore<T = Self> {
    fn new_base() -> Vec<Vec<Block>>;
    fn get_amphipod_space(&self, i: usize) -> &Block;
    fn get_amphipod_space_mut(&mut self, i: usize) -> &mut Block;
    fn init(&mut self, amphipods: impl ExactSizeIterator<Item=Amphipod>);
    fn get_move_coordinates() -> Vec<Coord>;
    fn is_locked(&self, coord: &Coord) -> bool;
    fn next_possible_amphipod_moves(&self) -> Vec<Coord>;
    fn move_amphipod(&mut self, from_coord: &Coord, to_coord: &Coord);
    fn get_next_possible_states(self) -> Vec<T>;
    fn is_complete(&self) -> bool;
    fn print(&self);
    fn possible_destinations(&self, coord: &Coord) -> Vec<Coord>;
}

impl<T: AmphipodBurrow + Clone> BurrowCore for T {
    fn new_base() -> Vec<Vec<Block>> {
        let wall = Block::wall();
        let non_stop_empty = Block::empty_space(false);
        let valid_stop_empty = Block::empty_space(true);
        let mut map: Vec<Vec<Block>> = Default::default();
        let get_row = || Vec::with_capacity(13);

        map.push(vec![wall.clone(); 13]);

        let mut row = get_row();
        row.push(wall.clone());
        row.push(valid_stop_empty.clone());
        for _ in 0..4 {
            row.push(valid_stop_empty.clone());
            row.push(non_stop_empty.clone());
        }
        row.extend(vec![valid_stop_empty.clone(); 2]);
        row.push(wall.clone());
        map.push(row);

        for _ in 0..T::AMPHIPOD_ROW_COUNT {
            let mut row = get_row();
            row.extend(vec![Block::wall(); 3]);
            for i in 0..4 {
                let for_type = match i {
                    0 => AmphipodType::Amber,
                    1 => AmphipodType::Bronze,
                    2 => AmphipodType::Copper,
                    3 => AmphipodType::Desert,
                    _ => panic!("invalid operation")
                };
                row.push(Block::destination(for_type));
                row.push(Block::wall());
            }
            row.extend(vec![Block::wall(); 2]);
            map.push(row);
        }

        map.push(vec![wall.clone(); 13]);

        map
    }
    fn get_amphipod_space(&self, i: usize) -> &Block {
        // #############
        // #...........#
        // ###0#1#2#3###
        //   #4#5#6#7#
        //   #8#9#10#11#
        //   #12#13#14#15#
        //   #########
        let first = if i < 4 { 2 } else if i < 8 { 3 } else if i < 12 { 4 } else { 5 };
        let second = (i % 4) * 2 + 3;
        let coord = Coord(first, second);
        self.get_space(&coord)
    }
    fn get_amphipod_space_mut(&mut self, i: usize) -> &mut Block {
        let first = if i < 4 { 2 } else if i < 8 { 3 } else if i < 12 { 4 } else { 5 };
        let second = (i % 4) * 2 + 3;
        let coord = Coord(first, second);
        self.get_space_mut(&coord)
    }
    fn init(&mut self, amphipods: impl Iterator<Item=Amphipod>) {
        for (i, amphipod) in amphipods.into_iter().enumerate() {
            self.get_amphipod_space_mut(i).contents = BlockContents::Amphipod(amphipod);
        }
        if T::AMPHIPOD_ROW_COUNT == 4 {
            for i in 12..=15 {
                let space = self.get_amphipod_space(i);
                let amphipod = space.get_amphipod().unwrap();
                if amphipod.has_destination(space) {
                    self.get_amphipod_space_mut(i).get_amphipod_mut().unwrap().moves_left = 0;
                }
            }
            for i in 8..=11 {
                let space = self.get_amphipod_space(i);
                let amphipod = space.get_amphipod().unwrap();
                let space_lower = self.get_amphipod_space(i + 4);
                let amphipod_lower = space_lower.get_amphipod().unwrap();
                if amphipod_lower.moves_left == 0 && amphipod.has_destination(space) {
                    self.get_amphipod_space_mut(i).get_amphipod_mut().unwrap().moves_left = 0;
                }
            }
        }
        for i in 4..=7 {
            let space = self.get_amphipod_space(i);
            let amphipod = space.get_amphipod().unwrap();

            if T::AMPHIPOD_ROW_COUNT == 4 {
                let space_lower = self.get_amphipod_space(i + 4);
                let amphipod_lower = space_lower.get_amphipod().unwrap();
                if amphipod_lower.moves_left == 0 && amphipod.has_destination(space) {
                    self.get_amphipod_space_mut(i).get_amphipod_mut().unwrap().moves_left = 0;
                }
            } else {
                if amphipod.has_destination(space) {
                    self.get_amphipod_space_mut(i).get_amphipod_mut().unwrap().moves_left = 0;
                }
            }
        }
        for i in 0..=3 {
            let space = self.get_amphipod_space(i);
            let amphipod = space.get_amphipod().unwrap();
            let space_lower = self.get_amphipod_space(i + 4);
            let amphipod_lower = space_lower.get_amphipod().unwrap();
            if amphipod_lower.moves_left == 0 && amphipod.has_destination(space) {
                self.get_amphipod_space_mut(i).get_amphipod_mut().unwrap().moves_left = 0;
            }
        }
    }
    fn get_move_coordinates() -> Vec<Coord> {
        let mut results = Vec::with_capacity(11 + 4 * T::AMPHIPOD_ROW_COUNT);
        results.extend((1..=11).map(|i| Coord(1, i))
            .chain((2..=T::AMPHIPOD_ROW_COUNT + 1)
                .flat_map(move |first| (0..4)
                    .map(move |i| Coord(first, (i % 4) * 2 + 3)))));
        results
    }
    fn is_locked(&self, coord: &Coord) -> bool {
        match coord.0 {
            1 => {
                let neighbors = [Coord(1, coord.1 - 1), Coord(1, coord.1 + 1)];
                neighbors.iter().all(|n| !self.get_space(n).is_empty())
            }
            2 => false,
            3 | 4 | 5 => {
                (2..=coord.0 - 1).any(|i| !self.get_space(&Coord(i, coord.1)).is_empty())
            }
            _ => panic!("out of range: is_locked")
        }
    }
    fn next_possible_amphipod_moves(&self) -> Vec<Coord> {
        Self::get_move_coordinates()
            .into_iter()
            .filter(|coord| {
                if let Some(amphipod) = self.get_space(&coord).get_amphipod() {
                    if amphipod.moves_left > 0 && !self.is_locked(&coord) {
                        return true;
                    }
                }
                false
            })
            .collect()
    }
    fn move_amphipod(&mut self, from_coord: &Coord, to_coord: &Coord) {
        let mut block = std::mem::take(&mut self.get_space_mut(from_coord).contents);
        if let BlockContents::Amphipod(amphipod) = &mut block {
            amphipod.moves_left -= 1;
            self.increase_cost(amphipod.get_move_cost() * from_coord.manhattan_distance(&to_coord))
        }
        self.get_space_mut(to_coord).contents = block;
    }
    fn get_next_possible_states(self) -> Vec<T> {
        if self.is_complete() {
            return vec![self];
        }
        let s = &self;
        s.next_possible_amphipod_moves()
            .into_iter()
            .flat_map(|coord| s.possible_destinations(&coord)
                .into_iter()
                .map(move |dest| {
                    let mut next_state = s.clone();
                    next_state.move_amphipod(&coord, &dest);
                    next_state
                }))
            .collect()
    }
    fn is_complete(&self) -> bool {
        Self::get_move_coordinates()
            .into_iter()
            .filter_map(|coord| self.get_space(&coord).get_amphipod())
            .all(|a| a.moves_left == 0)
    }
    fn print(&self) {
        for first in 0..=2 + T::AMPHIPOD_ROW_COUNT {
            let line: String = (0..13).map(|second| {
                let block = self.get_space(&Coord(first, second));
                match &block.contents {
                    BlockContents::Wall => '#',
                    BlockContents::EmptySpace => '.',
                    BlockContents::Amphipod(a) => match a.amphipod_type {
                        AmphipodType::Amber => 'A',
                        AmphipodType::Bronze => 'B',
                        AmphipodType::Copper => 'C',
                        AmphipodType::Desert => 'D'
                    }
                }
            }).collect();
            println!("{}", line);
        }
    }
    fn possible_destinations(&self, coord: &Coord) -> Vec<Coord> {
        let amphipod = self.get_space(coord).get_amphipod().unwrap();
        let mut destinations: Vec<Coord> = if amphipod.moves_left == 2 {
            (1..=11).map(|i| Coord(1, i))
                .filter(|c| {
                    let space = self.get_space(c);
                    space.is_empty() && space.is_valid_stop
                })
                .collect()
        } else {
            let i = match amphipod.amphipod_type {
                AmphipodType::Amber => 3,
                AmphipodType::Bronze => 5,
                AmphipodType::Copper => 7,
                AmphipodType::Desert => 9
            };
            let coords: Vec<Coord> = (2..=T::AMPHIPOD_ROW_COUNT + 1).map(|f| Coord(f, i)).collect();
            let spaces: Vec<&Block> = coords.iter().map(|c| self.get_space(c)).collect();

            (0..T::AMPHIPOD_ROW_COUNT)
                .filter_map(|j| if !spaces[j].is_empty() {
                    Some(vec![])
                } else if j == T::AMPHIPOD_ROW_COUNT - 1 || spaces[j + 1].is_finalized() {
                    Some(vec![coords[j].clone()])
                } else {
                    None
                })
                .next().unwrap()
        };
        destinations.retain(|dest| {
            let mut current = coord.clone();
            let is_ok = |c: &Coord| self.get_space(c).is_empty();
            current.0 = 1;
            let next = |c: &mut Coord| if dest.1 > coord.1 { c.1 += 1; } else { c.1 -= 1; };
            while current.1 != dest.1 {
                next(&mut current);
                if !is_ok(&current) {
                    return false;
                }
            }

            true
        });
        destinations
    }
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct BurrowV2(Vec<Vec<Block>>, usize);

impl AmphipodBurrow for BurrowV2 {
    fn get_cost(&self) -> usize {
        self.1
    }
    fn get_space(&self, coord: &Coord) -> &Block {
        &self.0[coord.0][coord.1]
    }

    fn get_space_mut(&mut self, coord: &Coord) -> &mut Block {
        &mut self.0[coord.0][coord.1]
    }

    fn increase_cost(&mut self, amount: usize) {
        self.1 += amount
    }

    const AMPHIPOD_ROW_COUNT: usize = 4;

    fn new() -> Self {
        Self(Self::new_base(), 0)
    }
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct Burrow(Vec<Vec<Block>>, usize);

impl AmphipodBurrow for Burrow {
    fn get_cost(&self) -> usize {
        self.1
    }

    fn get_space(&self, coord: &Coord) -> &Block {
        &self.0[coord.0][coord.1]
    }

    fn get_space_mut(&mut self, coord: &Coord) -> &mut Block {
        &mut self.0[coord.0][coord.1]
    }

    fn increase_cost(&mut self, amount: usize) {
        self.1 += amount
    }

    const AMPHIPOD_ROW_COUNT: usize = 2;

    fn new() -> Self {
        Self(Self::new_base(), 0)
    }
}

#[derive(PartialEq, Clone)]
struct Coord(usize, usize);

impl Coord {
    pub fn manhattan_distance(&self, other: &Coord) -> usize {
        self.0.max(other.0) - self.0.min(other.0) +
            self.1.max(other.1) - self.1.min(other.1)
    }
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
enum BlockContents {
    EmptySpace,
    Wall,
    Amphipod(Amphipod),
}

impl Default for BlockContents {
    fn default() -> Self {
        Self::EmptySpace
    }
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct Block {
    contents: BlockContents,
    is_destination_for: Option<AmphipodType>,
    is_valid_stop: bool,
}

impl Block {
    pub fn wall() -> Self {
        Self {
            contents: BlockContents::Wall,
            is_destination_for: None,
            is_valid_stop: false,
        }
    }
    pub fn get_amphipod(&self) -> Option<&Amphipod> {
        if let BlockContents::Amphipod(amphipod) = &self.contents {
            Some(amphipod)
        } else {
            None
        }
    }
    pub fn get_amphipod_mut(&mut self) -> Option<&mut Amphipod> {
        if let BlockContents::Amphipod(amphipod) = &mut self.contents {
            Some(amphipod)
        } else {
            None
        }
    }
    pub fn empty_space(is_valid_stop: bool) -> Self {
        Self {
            contents: BlockContents::EmptySpace,
            is_destination_for: None,
            is_valid_stop,
        }
    }
    pub fn destination(for_type: AmphipodType) -> Self {
        Self {
            contents: BlockContents::EmptySpace,
            is_destination_for: Some(for_type),
            is_valid_stop: false,
        }
    }
    pub fn is_finalized(&self) -> bool {
        if let BlockContents::Amphipod(amphipod) = &self.contents {
            amphipod.moves_left == 0
        } else {
            false
        }
    }
    pub fn is_empty(&self) -> bool {
        if let BlockContents::EmptySpace = self.contents {
            true
        } else {
            false
        }
    }
}

#[derive(Clone, PartialEq, Debug, Hash, Eq)]
enum AmphipodType {
    Amber,
    Bronze,
    Copper,
    Desert,
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct Amphipod {
    amphipod_type: AmphipodType,
    moves_left: usize,
}

impl Amphipod {
    pub fn new(c: char) -> Self {
        let amphipod_type = match c {
            'A' => AmphipodType::Amber,
            'B' => AmphipodType::Bronze,
            'C' => AmphipodType::Copper,
            'D' => AmphipodType::Desert,
            _ => panic!("out of bounds")
        };
        Self {
            amphipod_type,
            moves_left: 2,
        }
    }
    #[inline]
    fn get_move_cost(&self) -> usize {
        match self.amphipod_type {
            AmphipodType::Amber => 1,
            AmphipodType::Bronze => 10,
            AmphipodType::Copper => 100,
            AmphipodType::Desert => 1000
        }
    }
    pub fn has_destination(&self, block: &Block) -> bool {
        if let Some(for_type) = &block.is_destination_for {
            if self.amphipod_type == *for_type {
                return true;
            }
        }
        false
    }
}

fn _get_input() -> &'static str {
    "#############
#...........#
###D#C#D#B###
  #B#A#A#C#
  #########"
}