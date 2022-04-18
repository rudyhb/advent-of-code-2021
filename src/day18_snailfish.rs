use std::fmt::{Debug, Formatter};
use std::ops::Add;
use std::str::FromStr;
use self::SnailfishNumberPosition::{Left, Right};

pub(crate) fn run() {
    let _input = "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
[7,[5,[[3,8],[1,4]]]]
[[2,[2,2]],[8,[8,1]]]
[2,9]
[1,[[[9,3],9],[[9,0],[0,7]]]]
[[[5,[7,4]],7],1]
[[[[4,2],2],6],[8,7]]";
    let _input = "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";
    // let _input = _get_input();

    let get_lines = || _input.split('\n').map(|line| line.parse::<SnailfishNumber>().unwrap());
    let mut lines = get_lines();
    let mut number = lines.next().unwrap();

    for rhs in lines {
        println!("  {:?}", number);
        println!("+ {:?}", rhs);
        number = number + rhs;
        println!("= {:?}", number);
        println!();
    }
    println!("magnitude: {}", number.magnitude());

    let numbers: Vec<SnailfishNumber> = get_lines().collect();
    let max = (0..numbers.len())
        .flat_map(|i| (0..numbers.len()).map(move |j| (i, j)))
        .map(|(i, j)| (&numbers[i] + &numbers[j]).magnitude())
        .max().unwrap();
    println!("max magnitude between 2: {}", max);
}

#[derive(Clone)]
struct SnailfishNumber {
    left: SnailfishValue,
    right: SnailfishValue,
}

#[derive(Copy, Clone, PartialEq)]
enum SnailfishNumberPosition {
    Left,
    Right,
}

impl Debug for SnailfishNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let get_str = |val: &SnailfishValue| {
            match val {
                SnailfishValue::Numeric(val) => format!("{}", val),
                SnailfishValue::Child(child) => format!("{:?}", child)
            }
        };
        write!(f, "[{},{}]", get_str(&self.left), get_str(&self.right))
    }
}

impl SnailfishNumber {
    pub fn reduce(&mut self) {
        loop {
            if self.try_explode(Default::default()) {
                continue;
            }
            if self.try_split() {
                continue;
            }
            break;
        }
    }

    fn try_explode(&mut self, path: Vec<SnailfishNumberPosition>) -> bool {
        let node = self.get_node_unwrap_child(&path);
        if let SnailfishValue::Child(_) = &mut node.left {
            if self.try_explode(path.iter().chain(vec![SnailfishNumberPosition::Left].iter()).copied().collect()) {
                return true;
            }
        }
        let node = self.get_node_unwrap_child(&path);
        if let SnailfishValue::Child(_) = &mut node.right {
            if self.try_explode(path.iter().chain(vec![SnailfishNumberPosition::Right].iter()).copied().collect()) {
                return true;
            }
        }

        if path.len() < 4 {
            return false;
        }

        let node = self.get_node_unwrap_child(&path);

        if let Self { left: SnailfishValue::Numeric(left), right: SnailfishValue::Numeric(right) } = node {
            let left = *left;
            let right = *right;
            if let Some(left_neighbor) = self.find_left(&path) {
                *left_neighbor += left;
            }
            if let Some(right_neighbor) = self.find_right(&path) {
                *right_neighbor += right;
            }

            let parent = self.get_node_unwrap_child(&path[..path.len() - 1]);
            let node = match path.last().unwrap() {
                SnailfishNumberPosition::Left => &mut parent.left,
                SnailfishNumberPosition::Right => &mut parent.right
            };

            *node = SnailfishValue::Numeric(0);

            return true;
        }

        false
    }

    fn get_node_unwrap_child(&mut self, path: &[SnailfishNumberPosition]) -> &mut SnailfishNumber {
        if path.is_empty() {
            return self;
        }
        let node = self.get_node(path);
        if let SnailfishValue::Child(child) = node {
            child
        } else {
            panic!("node is not a child")
        }
    }

    fn get_node(&mut self, path: &[SnailfishNumberPosition]) -> &mut SnailfishValue {
        let mut node = self;
        for &p in path[..path.len() - 1].iter() {
            if p == SnailfishNumberPosition::Left {
                if let SnailfishValue::Child(c) = &mut node.left {
                    node = c;
                } else {
                    panic!("invalid path");
                }
            } else {
                if let SnailfishValue::Child(c) = &mut node.right {
                    node = c;
                } else {
                    panic!("invalid path");
                }
            }
        }

        match path.last().unwrap() {
            Left => {
                &mut node.left
            }
            Right => {
                &mut node.right
            }
        }
    }

    fn get_path(path: &Vec<SnailfishNumberPosition>, position: SnailfishNumberPosition) -> Vec<SnailfishNumberPosition> {
        let other = match position {
            SnailfishNumberPosition::Left => Right,
            SnailfishNumberPosition::Right => Left
        };

        path.iter().rev().skip_while(|p| **p != other).skip(1).copied().collect::<Vec<SnailfishNumberPosition>>().into_iter().rev().chain(vec![position]).collect()
    }

    fn find_left(&mut self, path: &Vec<SnailfishNumberPosition>) -> Option<&mut u32> {
        if path.iter().all(|&p| p == SnailfishNumberPosition::Left) {
            return None;
        }
        let path = Self::get_path(path, SnailfishNumberPosition::Left);
        let mut node = self.get_node(&path);
        loop {
            match node {
                SnailfishValue::Numeric(value) => {
                    return Some(value);
                }
                SnailfishValue::Child(child) => {
                    node = &mut child.right;
                }
            }
        }
    }

    fn find_right(&mut self, path: &Vec<SnailfishNumberPosition>) -> Option<&mut u32> {
        if path.iter().all(|&p| p == SnailfishNumberPosition::Right) {
            return None;
        }
        let path = Self::get_path(path, SnailfishNumberPosition::Right);
        let mut node = self.get_node(&path);
        loop {
            match node {
                SnailfishValue::Numeric(value) => {
                    return Some(value);
                }
                SnailfishValue::Child(child) => {
                    node = &mut child.left;
                }
            }
        }
    }

    fn try_split(&mut self) -> bool {
        let try_split_side = |side: &mut SnailfishValue| {
            match side {
                SnailfishValue::Numeric(val) => {
                    if *val >= 10 {
                        *side = SnailfishValue::Child(Box::new(SnailfishNumber {
                            left: SnailfishValue::Numeric(*val / 2),
                            right: SnailfishValue::Numeric(*val / 2 + *val % 2),
                        }));
                        true
                    } else {
                        false
                    }
                }
                SnailfishValue::Child(child) => {
                    Self::try_split(child)
                }
            }
        };
        try_split_side(&mut self.left) || try_split_side(&mut self.right)
    }

    pub fn magnitude(&self) -> u32 {
        let get_magnitude = |val: &SnailfishValue| {
            match val {
                SnailfishValue::Numeric(val) => *val,
                SnailfishValue::Child(child) => child.magnitude()
            }
        };
        3 * get_magnitude(&self.left) + 2 * get_magnitude(&self.right)
    }
}

impl FromStr for SnailfishNumber {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        chars.next();

        let get_size = |chars: &mut std::str::Chars| {
            let mut size = 0usize;
            let mut open_count = 0usize;
            while let Some(c) = chars.next() {
                match c {
                    '[' => {
                        open_count += 1;
                    }
                    ']' => {
                        if open_count == 0 {
                            break;
                        }
                        open_count -= 1;
                    }
                    ',' => {
                        if open_count == 0 {
                            break;
                        }
                    }
                    _ => {}
                }
                size += 1;
            }

            size
        };

        let size_left = get_size(&mut chars);
        let size_right = get_size(&mut chars);

        let get_next_value = |slice: &str| {
            if slice.chars().all(|c| c.is_numeric()) {
                SnailfishValue::Numeric(slice.parse().unwrap())
            } else {
                SnailfishValue::Child(Box::new(slice.parse().unwrap()))
            }
        };

        let mut i = 1usize;
        let slice = &s[i..i + size_left];
        let left = get_next_value(slice);
        i += size_left + 1;
        let slice = &s[i..i + size_right];
        let right = get_next_value(slice);

        Ok(Self { left, right })
    }
}

impl Add for SnailfishNumber {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut result = Self {
            left: SnailfishValue::Child(Box::new(self)),
            right: SnailfishValue::Child(Box::new(rhs)),
        };

        result.reduce();

        result
    }
}

impl Add for &SnailfishNumber {
    type Output = SnailfishNumber;

    fn add(self, rhs: Self) -> Self::Output {
        let mut result = SnailfishNumber {
            left: SnailfishValue::Child(Box::new(self.clone())),
            right: SnailfishValue::Child(Box::new(rhs.clone())),
        };

        result.reduce();

        result
    }
}

#[derive(Clone)]
enum SnailfishValue {
    Numeric(u32),
    Child(Box<SnailfishNumber>),
}


fn _get_input() -> &'static str {
    "[3,[5,[7,[3,9]]]]
[[[[7,0],0],[2,[2,8]]],[[[7,8],1],3]]
[[[[2,7],0],7],4]
[[2,1],[9,0]]
[[[[7,1],[3,2]],[[9,8],5]],[2,7]]
[[[8,9],[[8,7],0]],[[[8,7],[6,3]],[[1,7],[8,9]]]]
[[8,6],[[9,[1,7]],[6,[3,9]]]]
[[2,[[5,6],6]],[[4,[5,9]],[3,[4,5]]]]
[[[[2,0],[1,1]],[6,6]],[[1,9],[[2,7],[6,8]]]]
[[[4,6],[[6,3],[3,9]]],[[[2,6],[6,1]],[[9,9],[1,5]]]]
[[[4,[3,1]],3],6]
[[0,[[5,2],8]],[1,[9,[4,3]]]]
[[[[8,6],[2,1]],[2,[8,6]]],[[[7,1],[3,9]],0]]
[[[[4,7],[2,7]],[[8,9],2]],[[[2,4],[7,2]],[3,7]]]
[[5,[2,2]],[[1,6],[[9,1],[5,0]]]]
[[5,[[1,2],[6,4]]],[6,8]]
[[[5,[1,7]],7],[7,[8,1]]]
[[1,9],[[0,3],[[6,7],[2,4]]]]
[1,[7,[[0,6],0]]]
[[[[5,7],9],[[3,2],7]],[[5,1],[9,9]]]
[[[[0,4],[9,6]],[[8,3],[7,4]]],[7,[6,2]]]
[[[[1,6],0],[[8,0],[3,4]]],[[3,[0,3]],4]]
[4,[[7,8],[4,[9,7]]]]
[[[2,[3,7]],5],[0,[9,9]]]
[[[2,0],[[5,8],[7,6]]],[[9,[6,2]],[3,2]]]
[[[3,1],3],[[[3,7],6],[9,8]]]
[[7,[[2,5],5]],[5,[3,[4,5]]]]
[[[6,7],6],[2,[[9,3],9]]]
[[[[5,6],7],[[3,2],5]],[[9,[4,3]],[3,8]]]
[0,7]
[[[4,6],[2,9]],[[[7,6],[5,1]],7]]
[[0,5],[[1,[4,1]],[[7,3],9]]]
[[[2,[3,8]],5],[[[5,9],8],[7,0]]]
[[[6,[8,6]],[[3,6],7]],[[2,1],[6,[7,5]]]]
[[2,[[6,3],[8,9]]],[[[5,6],4],[[7,0],1]]]
[[[[7,1],[5,6]],8],[[[8,9],4],[8,3]]]
[[[9,2],[1,0]],0]
[[5,[5,[8,5]]],4]
[[3,[5,[4,9]]],3]
[[8,[[7,7],6]],5]
[[4,[[5,1],1]],[1,[1,[9,8]]]]
[[[7,[3,6]],[[2,8],[4,7]]],[[[8,8],[4,0]],[2,4]]]
[[[[3,6],3],[0,9]],2]
[[2,8],[[8,[8,6]],[[1,1],[4,5]]]]
[[2,[1,[1,0]]],[[[6,2],[7,4]],[[7,1],6]]]
[3,[8,[7,[8,6]]]]
[[1,0],[[[0,4],[0,5]],[1,5]]]
[[[[5,0],4],[[7,8],[8,8]]],[[1,7],0]]
[1,[[[4,1],7],[6,[9,0]]]]
[[[1,8],2],[[5,5],[8,5]]]
[[4,[9,[0,6]]],[[[8,9],[4,5]],4]]
[[[[5,4],[1,7]],[[3,1],[7,9]]],[[[0,8],[4,7]],[[5,9],6]]]
[[[[8,0],9],4],[[7,[1,3]],5]]
[[[[5,0],6],[[6,1],8]],[[9,1],7]]
[[9,[6,[8,8]]],[7,[[7,1],6]]]
[[[5,[1,5]],[3,[4,2]]],[[[5,2],7],[[6,9],[2,8]]]]
[[[5,[5,5]],[5,7]],[4,[[2,9],7]]]
[[[[0,4],0],[[0,6],[3,0]]],[0,[[8,1],2]]]
[[[7,[4,6]],[[7,2],[4,6]]],[[[9,3],[4,9]],6]]
[[6,7],7]
[[[4,1],[8,[1,5]]],[[4,6],0]]
[[[4,[5,5]],5],[[0,[2,7]],[1,1]]]
[[[[0,1],3],[6,7]],[4,7]]
[[4,[6,4]],[[[9,8],1],[9,3]]]
[[[4,9],0],[[[7,0],[0,9]],[1,[1,0]]]]
[[[7,9],[[9,5],[6,9]]],[[0,[3,0]],[0,[5,9]]]]
[9,[[0,0],[[1,9],9]]]
[[[5,[0,5]],[[9,8],[9,5]]],[[0,[2,5]],7]]
[[[[5,8],6],9],[[[2,7],7],[[7,8],5]]]
[[8,[[4,7],6]],2]
[[[[7,1],[9,0]],[9,[1,7]]],[[8,[6,7]],[2,5]]]
[[4,[2,9]],8]
[[[[7,6],[5,3]],[5,[9,7]]],[[6,[8,1]],[[6,4],9]]]
[[7,[[7,8],4]],[[1,3],[4,[9,7]]]]
[[[6,[6,7]],[[2,8],3]],[7,[6,[0,3]]]]
[[9,8],[[0,[4,8]],[[9,1],1]]]
[[[[4,0],[5,9]],7],[6,[[5,9],[9,6]]]]
[[8,1],[1,[9,[8,3]]]]
[[[1,[5,1]],[6,7]],[[5,9],[2,[6,7]]]]
[[[3,7],[[7,8],1]],[[0,[6,3]],[8,0]]]
[[5,[[9,3],[1,2]]],7]
[[[1,[9,9]],3],[[6,4],[4,1]]]
[[6,[1,[3,6]]],[2,9]]
[[2,[0,2]],[5,[[9,4],[5,0]]]]
[[4,[[3,1],[7,0]]],[[9,1],[[5,5],[6,7]]]]
[[3,[[7,1],[3,4]]],[7,[9,[9,4]]]]
[[9,9],[[5,4],[[9,7],4]]]
[[[5,1],8],[[6,7],9]]
[[[0,[9,5]],[4,3]],[3,2]]
[[[6,[4,1]],[[8,7],[5,3]]],[[[1,2],5],[[9,2],5]]]
[[[[7,4],[9,0]],[[1,8],[2,9]]],[[5,[1,9]],[4,0]]]
[[[4,[3,8]],[[3,3],[2,8]]],[[[1,3],9],[[8,5],6]]]
[[[[6,4],[7,9]],[[7,6],8]],[7,[9,8]]]
[[7,[3,5]],7]
[[[[5,0],[2,3]],[3,7]],[[4,[6,3]],[7,[4,4]]]]
[[6,[3,[7,6]]],[[[5,8],[8,1]],[3,[1,5]]]]
[[8,[9,[5,2]]],2]
[[1,[5,4]],[[7,[8,0]],8]]
[[[[2,7],4],3],[[1,4],[8,4]]]
[3,[9,2]]"
}