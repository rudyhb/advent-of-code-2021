use std::collections::HashMap;
use std::str::FromStr;

pub(crate) fn run() {
    let _input = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";
    let _input = _get_input();
    let mut lines = _input.split('\n');
    let mut template: TemplateV2 = lines.next().unwrap().parse().unwrap();
    lines.next();
    let rules: Rules = Rules::from_iter(lines);

    // println!("template: {}", template.0);
    println!("template: {:?}", template);

    for _i in 1..=40 {
        template.next(&rules);
        // println!("after step {}: {}", _i, template.0);
        println!("{:?}", template);
    }
    println!("score: {}", template.score());

}

struct Rules<'a>(HashMap<&'a str, &'a str>);

impl<'a> FromIterator<&'a str> for Rules<'a> {
    fn from_iter<T: IntoIterator<Item=&'a str>>(iter: T) -> Self {
        let mut map: HashMap<&'a str, &'a str> = Default::default();
        for line in iter {
            let mut parts = line.split_whitespace();
            let key = parts.next().unwrap();
            parts.next();
            let val = parts.next().unwrap();
            map.insert(key, val);
        }
        Self(map)
    }
}

struct Template(String);

impl FromStr for Template {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_string()))
    }
}

impl Template {
    pub fn _next(&mut self, rules: &Rules) {
        let mut next: String = Default::default();
        let mut chars = self.0.chars();
        let mut last_char = chars.next().unwrap();
        next.push(last_char);
        for c in chars {
            let key: String = vec![last_char, c].iter().collect();
            if let Some(val) = rules.0.get(key.as_str()) {
                next.push_str(val);
            }
            last_char = c;
            next.push(last_char);
        }

        self.0 = next;
    }
    pub fn _score(&self) -> i32 {
        let quantities: Vec<i32> = self.0.chars().fold(HashMap::new(), |mut acc, next| {
            let val = acc.entry(next).or_insert(0i32);
            *val += 1;
            acc
        }).values().copied().collect();
        quantities.iter().max().unwrap() - quantities.iter().min().unwrap()
    }
}

#[derive(Debug)]
struct TemplateV2(HashMap<String, u64>);

impl FromStr for TemplateV2 {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map: HashMap<String, u64> = Default::default();
        let mut chars = s.chars();
        let mut last_char = chars.next().unwrap();
        for c in chars {
            let key: String = vec![last_char, c].iter().collect();
            let val = map.entry(key).or_insert(0u64);
            *val += 1;
            last_char = c;
        }
        map.insert(last_char.to_string(), 1);
        Ok(Self(map))
    }
}

impl TemplateV2 {
    pub fn next(&mut self, rules: &Rules) {
        let mut next: HashMap<String, u64> = Default::default();
        let add = |map: &mut HashMap<String, u64>, key: String, amount: u64| {
            let val = map.entry(key).or_insert(0);
            *val += amount;
        };
        for (key, amount) in self.0.iter() {
            if let Some(next_letter) = rules.0.get(key.as_str()) {
                let mut chars = key.chars();
                let key1: String = vec![chars.next().unwrap()].into_iter().chain(next_letter.chars().into_iter()).collect();
                add(&mut next, key1, *amount);
                if let Some(second_char) = chars.next() {
                    let key2: String = next_letter.chars().into_iter().chain(vec![second_char].into_iter()).collect();
                    add(&mut next, key2, *amount);
                }
            } else {
                add(&mut next, key.clone(), *amount);
            }
        }
        self.0 = next;
    }
    pub fn score(&self) -> u64 {
        let quantities: Vec<u64> = self.0.iter().fold(HashMap::new(), |mut acc, (val, amount)| {
            let val = acc.entry(val.chars().next().unwrap()).or_insert(064);
            *val += *amount;
            acc
        }).values().copied().collect();
        quantities.iter().max().unwrap() - quantities.iter().min().unwrap()
    }
}

fn _get_input() -> &'static str {
    "PSVVKKCNBPNBBHNSFKBO

CF -> H
PP -> H
SP -> V
NO -> C
SF -> F
FS -> H
OF -> P
PN -> B
SH -> V
BO -> K
ON -> V
VP -> S
HN -> B
PS -> P
FV -> H
NC -> N
FN -> S
PF -> F
BF -> F
NB -> O
HS -> C
SC -> V
PC -> K
KF -> K
HC -> C
OK -> H
KS -> P
VF -> C
NV -> S
KK -> F
HV -> H
SV -> V
KC -> N
HF -> P
SN -> F
VS -> P
VN -> F
VH -> C
OB -> K
VV -> O
VC -> O
KP -> V
OP -> C
HO -> S
NP -> K
HB -> C
CS -> S
OO -> S
CV -> K
BS -> F
BH -> P
HP -> P
PK -> B
BB -> H
PV -> N
VO -> P
SS -> B
CC -> F
BC -> V
FF -> S
HK -> V
OH -> N
BV -> C
CP -> F
KN -> K
NN -> S
FB -> F
PH -> O
FH -> N
FK -> P
CK -> V
CN -> S
BP -> K
CH -> F
FP -> K
HH -> N
NF -> C
VB -> B
FO -> N
PB -> C
KH -> K
PO -> K
OV -> F
NH -> H
KV -> B
OS -> K
OC -> K
FC -> H
SO -> H
KO -> P
NS -> F
CB -> C
CO -> F
KB -> V
BK -> K
NK -> O
SK -> C
SB -> B
VK -> O
BN -> H"
}