use std::collections::HashMap;
use std::str::FromStr;

pub(crate) fn run() {
    // let input = "3,4,3,1,2";
    let input = get_input();

    let school: School = input.parse().unwrap();
    println!("initial state: {}", school._get_state());

    const NUM_DAYS: u32 = 256;

    // for _i in 0..NUM_DAYS {
    //     school._advance_day();
    //     // println!("after {} days: {}", _i + 1, school._get_state());
    //     // println!("{}", _i);
    // }
    //
    // println!("after {} days, there are {} fish", NUM_DAYS, school._size());

    println!("after {} days, there are {} fish", NUM_DAYS, school.size_after(NUM_DAYS as i32));
}

struct Lanternfish(u8);

const ROUND_TIMER: i32 = 7;
const NEWBORN_TIMER: i32 = 9;

struct CalcCache(HashMap<i32, u64>);

impl Default for CalcCache {
    fn default() -> Self {
        Self(Default::default())
    }
}
impl CalcCache {
    pub fn get(&self, key: i32) -> Option<&u64> {
        self.0.get(&key)
    }
    pub fn set(&mut self, key: i32, value: u64) {
        self.0.insert(key, value);
    }
}

impl Lanternfish {
    pub fn _advance_day(&mut self) -> Option<Lanternfish> {
        if self.0 == 0 {
            self.0 = ROUND_TIMER as u8 - 1;
            Some(Lanternfish(NEWBORN_TIMER as u8 - 1))
        } else {
            self.0 -= 1;
            None
        }
    }

    pub fn count_after_days(&self, num_days: i32, cache: &mut CalcCache) -> u64 {
        Self::do_count_after_days(self.0 as i32, num_days, cache)
    }
    fn do_count_after_days(state: i32, num_days: i32, cache: &mut CalcCache) -> u64 {
        let mut count = 1u64;
        let mut num_days = num_days;
        num_days -= state;
        let start_num_days = num_days;
        if let Some(value) = cache.get(start_num_days) {
            return *value;
        }
        while num_days > 0 {
            // println!("{}", num_days);
            count += Self::do_count_after_days(NEWBORN_TIMER, num_days, cache);
            num_days -= ROUND_TIMER;
        }

        cache.set(start_num_days, count);

        count
    }
}


impl FromStr for Lanternfish {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let i: u8 = s.parse().or(Err(()))?;
        Ok(Lanternfish(i))
    }
}

struct School(Vec<Lanternfish>);

impl FromStr for School {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(School(s.split(',')
            .map(|s| s.parse())
            .collect::<Result<Vec<Lanternfish>, _>>().or(Err(()))?))
    }
}

impl School {
    pub fn _advance_day(&mut self) {
        let mut new_fish: Vec<Lanternfish> = Vec::with_capacity(self.0.len() / (ROUND_TIMER as usize / 2));

        for fish in self.0.iter_mut() {
            if let Some(newborn) = fish._advance_day() {
                new_fish.push(newborn);
            }
        }

        self.0.append(&mut new_fish);
    }
    pub fn _size(&self) -> usize {
        self.0.len()
    }
    pub fn _get_state(&self) -> String {
        self.0.iter()
            .flat_map(|i| i.0.to_string().chars().chain(vec![',']).collect::<Vec<char>>())
            .collect::<String>()
    }
    pub fn size_after(&self, num_days: i32) -> u64 {
        let mut cache: CalcCache = Default::default();
        self.0.iter().map(|fish| fish.count_after_days(num_days, &mut cache)).sum()
    }
}

impl School {}

fn get_input() -> &'static str {
    "3,1,5,4,4,4,5,3,4,4,1,4,2,3,1,3,3,2,3,2,5,1,1,4,4,3,2,4,2,4,1,5,3,3,2,2,2,5,5,1,3,4,5,1,5,5,1,1,1,4,3,2,3,3,3,4,4,4,5,5,1,3,3,5,4,5,5,5,1,1,2,4,3,4,5,4,5,2,2,3,5,2,1,2,4,3,5,1,3,1,4,4,1,3,2,3,2,4,5,2,4,1,4,3,1,3,1,5,1,3,5,4,3,1,5,3,3,5,4,2,3,4,1,2,1,1,4,4,4,3,1,1,1,1,1,4,2,5,1,1,2,1,5,3,4,1,5,4,1,3,3,1,4,4,5,3,1,1,3,3,3,1,1,5,4,2,5,1,1,5,5,1,4,2,2,5,3,1,1,3,3,5,3,3,2,4,3,2,5,2,5,4,5,4,3,2,4,3,5,1,2,2,4,3,1,5,5,1,3,1,3,2,2,4,5,4,2,3,2,3,4,1,3,4,2,5,4,4,2,2,1,4,1,5,1,5,4,3,3,3,3,3,5,2,1,5,5,3,5,2,1,1,4,2,2,5,1,4,3,3,4,4,2,3,2,1,3,1,5,2,1,5,1,3,1,4,2,4,5,1,4,5,5,3,5,1,5,4,1,3,4,1,1,4,5,5,2,1,3,3"
}