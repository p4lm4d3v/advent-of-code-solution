use std::collections::HashMap;

mod test;

fn main() {
    let input = include_str!("./input2.txt");
    let output = process(input);
    dbg!("{}", output);
}

fn process(input: &str) -> i32 {
    let lines = input.lines().collect::<Vec<_>>();
    let len = lines.len();

    let mut left: Vec<i32> = vec![];
    let mut right: Vec<i32> = vec![];

    for &line in lines.iter() {
        let (a, b) = parse_line(line);
        left.push(a);
        right.push(b);
    }

    let mut counts: Vec<i32> = vec![];

    for i in 0..len {
        let count = count_in(left[i], &right);
        counts.push(count);
    }

    let mut score = 0;

    for i in 0..len {
        score += left[i] * counts[i];
    }

    score
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

fn count_in(n: i32, nums: &Vec<i32>) -> i32 {
    let mut map: HashMap<i32, i32> = HashMap::new();

    for num in nums {
        if map.contains_key(&num) {
            let val = map.get(&num).unwrap();
            map.insert(*num, val + 1);
        } else {
            map.insert(*num, 1);
        }
    }

    *map.get(&n).unwrap_or(&0)
}
