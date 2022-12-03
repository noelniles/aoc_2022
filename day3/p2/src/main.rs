use std::io::{self, BufReader, prelude::*};
use std::fs::File;
use std::collections::{HashMap, HashSet};
use itertools::Itertools;


fn intersection(mut sets: Vec<HashSet<char>>) -> HashSet<char> {
    if sets.is_empty() {
        return HashSet::new();
    }
    if sets.len() == 1 {
        return sets.pop().unwrap();
    }

    let mut res = sets.pop().unwrap();
    res.retain(|i| {
        sets.iter().all(|s| s.contains(i))
    });
    res
}

fn main() -> io::Result<()> {

    let lowers = "abcdefghijklmnopqrstuvwxyz".to_string();
    let uppers = lowers.to_uppercase();

    let mut lower_chars: Vec<char> = lowers.chars().collect();
    let mut upper_chars: Vec<char> = uppers.chars().collect();

    lower_chars.append(&mut upper_chars);

    let mut map = HashMap::new();

    for (i, x) in lower_chars.iter().enumerate() {
        map.insert(x, i+1);
    }

    let f         = File::open("../data.txt")?;
    let reader    = BufReader::new(f);
    let mut total = 0;

    for line in &reader.lines().chunks(3) {
        let lines: Vec<_> = line.collect();
        let a: HashSet<char> = lines[0].as_ref().unwrap().chars().collect();
        let b: HashSet<char> = lines[1].as_ref().unwrap().chars().collect();
        let c: HashSet<char> = lines[2].as_ref().unwrap().chars().collect();

        let sets = vec!(a, b, c);
        let intersect = intersection(sets);
        assert_eq!(intersect.len(), 1);
        let index = intersect.iter().nth(0).unwrap();
        total += map[index];
    }
    println!("total {}", total);

    Ok(())
}
