mod test;

fn main() {
    let input = include_str!("./input1.txt");
    let output = process(input);
    dbg!("{}", output);
}

fn process(input: &str) -> i32 {
    let lines = input.lines().collect::<Vec<_>>();

    let mut left: Vec<i32> = vec![];
    let mut right: Vec<i32> = vec![];

    for &line in lines.iter() {
        let (a, b) = parse_line(line);
        left.push(a);
        right.push(b);
    }

    left.sort();
    right.sort();

    let mut sum = 0;

    for i in 0..lines.len() {
        let a = left[i];
        let b = right[i];
        sum += (a - b).abs();
    }

    sum
}

fn parse_line(line: &str) -> (i32, i32) {
    let split = line.split("   ").collect::<Vec<_>>();

    let a: i32 = split[0]
        .parse()
        .expect(format!("Couldn't parse \"{}\"!", split[0]).as_str());
    let b: i32 = split[1]
        .parse()
        .expect(format!("Couldn't parse \"{}\"!", split[1]).as_str());

    (a, b)
}
