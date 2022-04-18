use std::collections::{HashMap, HashSet};

pub(crate) fn run() {
    let _input = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";
    let _input = "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";
    let _input = "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW";
    let _input = _get_input();

    let network: Network = _input.into();

    let valid_paths = get_paths(&network, &vec![]);
    for path in valid_paths.iter() {
        println!("{:?}", path);
    }
    println!("there are {} valid paths", valid_paths.len());

}

fn get_paths<'a>(network: &'a Network<'a>, parent: &Vec<&'a str>) -> Vec<Vec<&'a str>> {
    let current_cave = if parent.is_empty() {
        "start"
    } else {
        parent.last().unwrap()
    };

    let _is_valid_path_v1 = |path: &Vec<&'a str>| -> bool {
        let mut set: HashSet<&'a str> = Default::default();
        for p in path.iter() {
            if p.chars().next().unwrap().is_lowercase() {
                if set.contains(p) {
                    return false;
                }
                set.insert(p);
            }
        }
        true
    };
    let _is_valid_path_v2 = |path: &Vec<&'a str>| -> bool {
        let mut set: HashSet<&'a str> = Default::default();
        let mut visited_twice = false;
        for p in path.iter() {
            if p.chars().next().unwrap().is_lowercase() {
                if set.contains(p) {
                    if visited_twice {
                        return false;
                    } else {
                        visited_twice = true;
                    }
                }
                set.insert(p);
            }
        }
        true
    };

    network.paths_from(current_cave)
        .filter_map(|&next| {
            let mut next_path = parent.clone();
            next_path.push(next);
            if next == "end" {
                Some(vec![next_path])
            } else if !_is_valid_path_v2(&next_path) {
                None
            } else {
                Some(get_paths(network, &next_path))
            }
        })
        .flat_map(|paths| paths.into_iter())
        .collect::<Vec<Vec<&'a str>>>()
}

struct Network<'a>(HashMap<&'a str, HashSet<&'a str>>);

impl<'a> Network<'a> {
    pub fn paths_from(&self, cave: &'a str) -> impl Iterator<Item=&&str> {
        self.0.get(cave).unwrap().into_iter()
    }
}

impl<'a> From<&'a str> for Network<'a> {
    fn from(s: &'a str) -> Self {
        let mut map: HashMap<&'a str, HashSet<&'a str>> = Default::default();
        for line in s.split('\n') {
            let mut parts = line.split('-');
            let left = parts.next().unwrap();
            let right = parts.next().unwrap();
            for items in [(left, right), (right, left)] {
                if items.1 == "start" || items.0 == "end" {
                    continue;
                }
                if !map.contains_key(items.0) {
                    map.insert(items.0, Default::default());
                }
                let values = map.get_mut(items.0).unwrap();
                values.insert(items.1);
            }
        }

        Self(map)
    }
}



fn _get_input() -> &'static str {
    "mx-IQ
mx-HO
xq-start
start-HO
IE-qc
HO-end
oz-xq
HO-ni
ni-oz
ni-MU
sa-IE
IE-ni
end-sa
oz-sa
MU-start
MU-sa
oz-IE
HO-xq
MU-xq
IE-end
MU-mx"
}