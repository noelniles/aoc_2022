use std::io::{self, BufReader, prelude::*};
use std::fs::File;
use std::collections::{HashMap, HashSet};

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

    let f         = File::open("./data.txt")?;
    let reader    = BufReader::new(f);
    let mut total = 0;

    for line in reader.lines() {
        let s   = line.unwrap();
        let mid = s.chars().count() / 2;
        let s   = s.split_at(mid);

        let a: HashSet<char> = s.0.chars().collect();
        let b: HashSet<char> = s.1.chars().collect();
        let u = a.intersection(&b).nth(0).unwrap();
        total += map[&u];
    }
    println!("total {}", total);

    Ok(())
}
