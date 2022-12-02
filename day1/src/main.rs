use std::env;
use std::io::{self, BufReader, prelude::*};
use std::fs::File;


/// Returns the n largest values in v in ascending order.
fn nmax(n: usize, v: &mut [u32]) -> Vec<u32> {
    v.sort();
    v[v.len() - n..v.len()].to_vec()
}

fn main() -> io::Result<()> {
    let args = env::args().collect::<Vec<String>>();

    if args.len() < 2 {
        println!("Usage: {} N", args[0]);
        println!("This will return the N largest items in the list");
        panic!("Missing argument N");
    }

    let f         = File::open("./data.txt")?;
    let reader    = BufReader::new(f);
    let mut cals  = Vec::new();
    let mut total = 0;
    let n: usize  = args[1].parse().expect("Unable to parse N as a number");

    for line in reader.lines() {
        let s = line.unwrap();

        if s.is_empty() {
            cals.push(total);
            total = 0;
            continue;
        }
        total += s.parse::<u32>().unwrap();
    }
    let res = nmax(n, cals.as_mut_slice());

    println!("{:?}", res);

    Ok(())
}
