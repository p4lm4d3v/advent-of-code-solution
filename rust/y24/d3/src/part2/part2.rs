extern crate regex;

use regex::Regex;

mod test;

fn main() {
    let input = include_str!("./input2.txt");
    let output = process(input);
    dbg!("{}", output);
}

fn process(input: &str) -> i32 {
    let rx = Regex::new(r"(?:mul\(\d+,\d+\)|don't\(\)|do\(\))").unwrap();
    let matches = rx.find_iter(input);

    let mut sum = 0;
    let mut mul: bool = true;

    for mat in matches {
        let mat = mat.as_str();

        if mat == "do()" {
            mul = true;
        } else if mat == "don't()" {
            mul = false;
        } else {
            if mul {
                let m = mat.replace("mul", "").replace("(", "").replace(")", "");
                let parts: Vec<&str> = m.split(",").collect();
                let a = parts[0].parse::<i32>().unwrap();
                let b = parts[1].parse::<i32>().unwrap();
                sum += a * b;
            }
        }
    }

    sum
}
