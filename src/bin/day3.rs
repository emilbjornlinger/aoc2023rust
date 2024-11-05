use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let path1 = Path::new("./src/input/test3_1.txt");

    let output1 = part1(&path1);
    dbg!(output1);

    let path2 = Path::new("./src/input/input3.txt");

    //let output2 = part2(&path2);
    //dbg!(output2);
}

fn part1(path: &Path) -> String {
    let display = path.display();
    let file = match File::open(&path) {
        Err(_) => panic!("couldn't open {}", display),
        Ok(file) => file,
    };

    let reader = BufReader::new(file);
    let lines = reader.lines();

    // Create data structure to store each number together with a "perimiter"
    // for that number, extract all the numbers and create the perimiters,
    // don't add perimiter values outside of the map
    // Then parse all symbols into a vector of vectors of u8 of fixed size
    // and put a 1 if a symbol is there and a 0 otherwise
    // Then iterate all of the perimiter values and do a lookup in the symbol
    // chart, and include if you hit a 1
}
