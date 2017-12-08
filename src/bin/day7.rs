#[macro_use]
extern crate failure;
#[macro_use]
extern crate lazy_static;
extern crate regex;

use failure::Error;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::string::String;

fn main() {
    if let Err(ref err) = run() {
        eprintln!("error: {:?}", err);
        eprintln!("bactrace: {:?}", err.backtrace());
        ::std::process::exit(1);
    }
}

fn run() -> Result<(), Error> {
    let mut file = File::open("input/day7.txt")?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;

    let towers = parse(&input)?;

    println!("a: {:?}", run_a(&towers));
    println!("b: {:?}", run_b(&towers));

    Ok(())
}

fn run_a(tree: &Towers) -> Option<&str> {
    root(&tree)
}

fn run_b(tree: &Towers) -> Option<u32> {
    if let Some(root) = root(&tree) {
        populate_all_energy(&tree, &root)
    } else {
        None
    }
}

fn root(tree: &Towers) -> Option<&str> {
    tree.iter()
        .find(|&(_, value)| value.parent.is_empty())
        .map(|(name, _)| name.as_str())
}


macro_rules! distance {
    ($x:expr, $y:expr) => (
        if $x < $y {
            $y - $x
        } else {
            $x - $y
        })
}

fn populate_all_energy<'a>(tree: &Towers, root: &'a str) -> Option<u32> {
    let mut accumulated_energy = HashMap::new();
    let mut stack: Vec<&str> = vec![root];

    while !stack.is_empty() {
        let &name = stack.last().unwrap();
        let tower = tree.get(name).unwrap();

        if tower.sub_towers.is_empty() {
            accumulated_energy.insert(name, tower.energy);
            stack.pop();
        } else {
            let children = tower
                .sub_towers
                .iter()
                .map(|t| *accumulated_energy.entry(t).or_insert(0))
                .collect::<Vec<u32>>();
            let is_equilibrium = children.iter().all(|e| *e == children[0]);
            let unbalanced = children.iter().find(|e| *e != &children[0]);
            let children_energy = children.iter().sum::<u32>();

            if unbalanced.is_none() && children_energy != 0 {
                println!("insert: {};{}", name, tower.energy + children_energy);
                accumulated_energy.insert(name, tower.energy + children_energy);
                stack.pop();
            } else if unbalanced.is_none() && children_energy == 0 {
                tower.sub_towers.iter().for_each(|t| stack.push(&t));
            } else {
                return Some(distance!(*unbalanced.unwrap(), children[0]));
            }
        }
    }

    None
}

type SubTower = Vec<String>;

#[derive(Debug, Hash, Eq, PartialEq)]
struct Tower {
    parent: String,
    energy: u32,
    sub_towers: SubTower,
}

type Towers = HashMap<String, Tower>;

fn parse(input: &str) -> Result<Towers, Error> {
    let mut towers: HashMap<String, Tower> = HashMap::new();
    for line in input.lines() {
        let mut it = line.split("->");

        if let Some(tower) = it.next() {
            let (name, energy) = parse_tower(tower.trim())?;

            let sub_towers = it.next().map_or(vec![], |sub_tower| {
                sub_tower.split(',').map(|t| t.trim().to_owned()).collect()
            });

            towers
                .entry(name.clone())
                .or_insert(Tower {
                    parent: String::new(),
                    energy,
                    sub_towers: sub_towers.clone(),
                })
                .energy = energy;

            towers.get_mut(&name).unwrap().sub_towers = sub_towers.clone();

            sub_towers.iter().for_each(|sub_tower| {
                towers
                    .entry(sub_tower.clone())
                    .or_insert(Tower {
                        parent: name.clone(),
                        energy: 0,
                        sub_towers: vec![],
                    })
                    .parent = name.clone()
            });
        } else {
            return Err(format_err!("Cannot read line after '->' split: {}", line));
        }
    }

    Ok(towers)
}

fn parse_tower(tower: &str) -> Result<(String, u32), Error> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(\w+) \((\d+)\)$").unwrap();
    }
    if let Some(captures) = RE.captures(tower) {
        Ok((captures[1].to_owned(), captures[2].parse()?))
    } else {
        Err(format_err!("Regex cannot match: {:?}", tower))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_a() {
        let input = "pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)";

        if let Ok(towers) = parse(&input) {
            assert_eq!(Some("tknk"), run_a(&towers));
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_run_b() {
        let input = "pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)";

        if let Ok(towers) = parse(&input) {
            assert_eq!(Some(8), run_b(&towers));
        } else {
            assert!(false);
        }
    }
}
