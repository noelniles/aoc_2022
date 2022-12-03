use std::collections::HashMap;
use std::io::{self, BufReader, prelude::*};
use std::fs::File;



fn main() -> io::Result<()> {
    let map = HashMap::from([
        ("A X".to_string(), 3), ("A Y".to_string(), 4), ("A Z".to_string(), 8),
        ("B X".to_string(), 1), ("B Y".to_string(), 5), ("B Z".to_string(), 9),
        ("C X".to_string(), 2), ("C Y".to_string(), 6), ("C Z".to_string(), 7),
    ]);

    let f = File::open("./data.txt")?;
    let reader = BufReader::new(f);
    let mut total = 0;

    for line in reader.lines() {
        total += map[&line?];
    }

    println!("{}", total);
    Ok(())
}
