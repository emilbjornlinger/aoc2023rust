use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let path1 = Path::new("./src/input/input2.txt");

    let output1 = part1(&path1);
    dbg!(output1);

    let path2 = Path::new("./src/input/input2.txt");

    let output2 = part2(&path2);
    dbg!(output2);
}

fn part1(path: &Path) -> String {
    let display = path.display();
    let file = match File::open(&path) {
        Err(_) => panic!("couldn't open {}", display),
        Ok(file) => file,
    };

    const RED_LIMIT: i32 = 12;
    const GREEN_LIMIT: i32 = 13;
    const BLUE_LIMIT: i32 = 14;

    let reader = BufReader::new(file);
    let lines = reader.lines();

    let mut counter: u32 = 0;

    for (idx, line) in lines.enumerate() {
        let mut is_possible = true;
        let line = line.expect("could not extract line");

        let parts: Vec<_> = line.split(':').collect();
        assert!(parts.len() == 2);

        'outer: for set in parts[1].split(';') {
            let mut vals: Vec<_> = set.split(' ').collect();
            vals.retain(|x| *x != "");
            for (val_idx, val) in vals.iter().enumerate().step_by(2) {
                match val.parse::<i32>() {
                    Ok(num) => {
                        let color_str = vals[val_idx + 1];
                        if color_str.starts_with("red") {
                            if num > RED_LIMIT {
                                is_possible = false;
                                break 'outer;
                            }
                        } else if color_str.starts_with("green") {
                            if num > GREEN_LIMIT {
                                is_possible = false;
                                break 'outer;
                            }
                        } else if color_str.starts_with("blue") {
                            if num > BLUE_LIMIT {
                                is_possible = false;
                                break 'outer;
                            }
                        } else {
                            panic!("unexpected color is unknown")
                        }
                    }
                    _ => panic!("could not convert to a number as expected"),
                }
            }
        }
        
        if is_possible {
            // idx + 1 equals game id
            counter += idx as u32 + 1;
        }
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

    let mut counter: u32 = 0;

    for (_idx, line) in lines.enumerate() {
        let line = line.expect("could not extract line");

        let parts: Vec<_> = line.split(':').collect();
        assert!(parts.len() == 2);

        let mut game_max_values = (0, 0, 0);

        for set in parts[1].split(';') {
            let mut vals: Vec<_> = set.split(' ').collect();
            vals.retain(|x| *x != "");
            for (val_idx, val) in vals.iter().enumerate().step_by(2) {
                match val.parse::<i32>() {
                    Ok(num) => {
                        let color_str = vals[val_idx + 1];
                        if color_str.starts_with("red") {
                            if num > game_max_values.0 {
                                game_max_values.0 = num;
                            }
                        } else if color_str.starts_with("green") {
                            if num > game_max_values.1 {
                                game_max_values.1 = num;
                            }
                        } else if color_str.starts_with("blue") {
                            if num > game_max_values.2 {
                                game_max_values.2 = num;
                            }
                        } else {
                            panic!("unexpected color is unknown")
                        }
                    }
                    _ => panic!("could not convert to a number as expected"),
                }
            }
        }
        
        counter += (game_max_values.0 * 
            game_max_values.1 * 
            game_max_values.2) as u32;
    }

    counter.to_string()
}
