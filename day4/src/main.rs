use std::fs::File;
use std::io::{self, BufReader, prelude::*};


struct Interval {
    lo: u32,
    hi: u32,    
}

impl Interval {
    #[allow(dead_code)]
    pub fn new(lo: u32, hi: u32) -> Self {
        Self { lo, hi, }
    }

    // Accepts a range in the format "lo-hi".
    fn from(range: &str) -> Self {
        assert!(range.contains("-"));
        let s: Vec<u32> = range.split("-").map(|x| x.parse::<u32>().unwrap()).collect();
        Self {
            lo: s[0],
            hi: s[1],
        }
    }

    fn overlap(&self, interval: &Interval) -> bool {
        (self.lo <= interval.hi) & (interval.lo <= self.hi)
    }

    fn contains(&self, interval: &Interval) -> bool {
        (self.lo <= interval.lo) & (self.hi >= interval.hi)
    }
}

fn main() -> io::Result<()> {
    let f = File::open("./data.txt")?;
    let reader = BufReader::new(f);
    let mut total_contains: usize = 0;
    let mut total_overlap: usize  = 0;

    for line in reader.lines() {
        let s = line.unwrap();
        let v: Vec<&str> = s.split(",").collect();
        let a = Interval::from(v[0]);
        let b: Interval = Interval::from(v[1]);

        if a.overlap(&b) {
            total_overlap += 1;
            if a.contains(&b) | b.contains(&a) {
                total_contains += 1;
            }
        }
    }
    
    println!("{} intervals completely contain the other", total_contains);
    println!("{} intervals overlap", total_overlap);
    Ok(())
}
