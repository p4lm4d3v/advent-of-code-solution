use std::vec;

mod test;

fn main() {
    let input = include_str!("./input1.txt");
    let output = process(input);
    dbg!("{}", output);
}

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Rising,
    Falling,
    Unknown,
}

fn process(input: &str) -> i32 {
    let lines = input.lines().collect::<Vec<_>>();

    let mut counter = 0;

    for line in lines {
        if is_safe(line) {
            counter += 1;
        }
    }

    counter
}

fn is_safe(line: &str) -> bool {
    let nums = parse_line(line);
    let steps = calculate_steps(nums);
    let rise_fall = steps_rise_fall(&steps) != Direction::Unknown;
    let in_interval = steps_in_interval(&steps);

    rise_fall && in_interval
}

fn parse_line(line: &str) -> Vec<i32> {
    line.trim()
        .split(" ")
        .map(|str| {
            str.trim()
                .parse::<i32>()
                .expect(format!("Couldn't parse '{}'", str).as_str())
        })
        .collect::<Vec<_>>()
}

fn calculate_steps(nums: Vec<i32>) -> Vec<i32> {
    let mut steps: Vec<i32> = vec![];

    for w in nums.windows(2) {
        let a = w[0];
        let b = w[1];
        steps.push(b - a);
    }

    steps
}

fn steps_rise_fall(steps: &Vec<i32>) -> Direction {
    if steps.contains(&0) {
        return Direction::Unknown;
    }

    let scaled_steps = steps
        .iter()
        .map(|&step| step / step.abs())
        .collect::<Vec<_>>();

    let sum: i32 = scaled_steps.iter().sum();
    let d = sum / scaled_steps.len() as i32;

    match d {
        1 => Direction::Rising,
        -1 => Direction::Falling,
        _ => Direction::Unknown,
    }
}

fn steps_in_interval(steps: &Vec<i32>) -> bool {
    for step in steps {
        if !vec![1, 2, 3].contains(&step.abs()) {
            return false;
        }
    }
    return true;
}
