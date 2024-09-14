use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;

enum Digits {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl Digits {
    fn string(&self) -> &str {
        match *self {
            Digits::One => "one",
            Digits::Two => "two",
            Digits::Three => "three",
            Digits::Four => "four",
            Digits::Five => "five",
            Digits::Six => "six",
            Digits::Seven => "seven",
            Digits::Eight => "eight",
            Digits::Nine => "nine",
        }
    }

    fn char(&self) -> char {
        match *self {
            Digits::One => '1',
            Digits::Two => '2',
            Digits::Three => '3',
            Digits::Four => '4',
            Digits::Five => '5',
            Digits::Six => '6',
            Digits::Seven => '7',
            Digits::Eight => '8',
            Digits::Nine => '9',
        }
    }
}

fn main() {
    let path1 = Path::new("./src/input/input1.txt");
    println!("The current directory is {}", env::current_dir().unwrap().display());

    let output1 = part1(&path1);
    dbg!(output1);

    let path2 = Path::new("./src/input/input1.txt");
    println!("The current directory is {}", env::current_dir().unwrap().display());

    let output2 = part2(&path2);
    dbg!(output2);
}

fn part1(path: &Path) -> String {
    let display = path.display();
    let file = match File::open(&path) {
        Err(_) => panic!("couldn't open {}", display),
        Ok(file) => file,
    };

    let reader = BufReader::new(file);
    let lines = reader.lines();

    let mut counter: u32 = 0;

    for line in lines {
        let mut line_num = String::from("");
        let line_first = line.unwrap();
        let line_second = line_first.clone();

        // Extract first digit from left to right
        for ch in line_first.chars() {
            match ch.to_digit(10) {
                Some(_) => {
                    line_num.push(ch);
                    break;
                }
                _ => (),
            };
        }

        // Extract first digit from right to left
        for ch in line_second.chars().rev() {
            match ch.to_digit(10) {
                Some(_) => {
                    line_num.push(ch);
                    break;
                }
                _ => (),
            };
        }

        let line_num = match line_num.parse::<u32>() {
            Ok(num) => num,
            _ => panic!("Tried to convert number that couldn't be converted"),
        };

        counter += line_num;
    }

    counter.to_string()
}

fn part2(path: &Path) -> String {
    let display = path.display();
    let file = match File::open(&path) {
        Err(_) => panic!("couldn't open {}", display),
        Ok(file) => file,
    };

    let reader = BufReader::new(file);
    let lines = reader.lines();

    let num_values = [
        Digits::One,
        Digits::Two,
        Digits::Three,
        Digits::Four,
        Digits::Five,
        Digits::Six,
        Digits::Seven,
        Digits::Eight,
        Digits::Nine,
    ];

    let mut counter: u32 = 0;

    for line in lines {
        let mut line_num = String::from("");
        let line_first = line.unwrap();
        let line_second = line_first.clone();

        // Extract first digit from left to right
        'outer: for (i, ch) in line_first.chars().enumerate() {
            
            match ch.to_digit(10) {
                Some(_) => {
                    line_num.push(ch);
                    break;
                }
                _ => {
                    for num in num_values.iter() {
                        if line_second[i..].starts_with(num.string()) {
                            line_num.push(num.char());
                            break 'outer;
                        }
                    }
                }
            };
        }

        // Extract first digit from right to left
        'outer: for (i, ch) in line_second.chars().rev().enumerate() {
            match ch.to_digit(10) {
                Some(_) => {
                    line_num.push(ch);
                    break;
                }
                _ => {
                    for num in num_values.iter() {
                        if line_second[(line_second.len() - 1 -i)..].starts_with(num.string()) {
                            line_num.push(num.char());
                            break 'outer;
                        }
                    }
                }
            };
        }

        let line_num = match line_num.parse::<u32>() {
            Ok(num) => num,
            _ => panic!("Tried to convert number that couldn't be converted"),
        };

        counter += line_num;
    }

    counter.to_string()
}
