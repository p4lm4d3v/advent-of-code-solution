extern crate regex;

use regex::Regex;

mod test;

fn main() {
    let input = include_str!("./input1.txt");
    let output = process(input);
    dbg!("{}", output);
}

fn process(input: &str) -> i32 {
    let rx = Regex::new(r"mul\(\d+,\d+\)").unwrap();
    let matches = rx.find_iter(input);

    let mut sum = 0;

    for mat in matches {
        let mat = mat
            .as_str()
            .replace("mul", "")
            .replace("(", "")
            .replace(")", "");
        let parts: Vec<&str> = mat.split(",").collect();
        let a = parts[0].parse::<i32>().unwrap();
        let b = parts[1].parse::<i32>().unwrap();
        sum += a * b;
    }

    sum
}
