fn main() {
    let input = include_str!("../input/input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    print!("{input}");
    "This is the output".to_string()
}

