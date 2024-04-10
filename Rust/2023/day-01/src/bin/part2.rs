use std::collections::HashMap;

fn main() {
    let input: &str = include_str!("./input2.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> i32 {
    let lines: Vec<&str> = input.lines().collect();
    let mut numbers: Vec<i32> = vec![];
    for l in lines {
        let line = l.trim();
        let parsed = parse(line, 0);
        let left = left_number(&parsed);
        let right = right_number(&parsed);
        let str = left.to_string() + &right.to_string();
        let num: i32 = str.parse().unwrap(); 
        numbers.push(num);
    }
    numbers.iter().sum()
}

fn parse(input: &str, iter_count: i32) -> String {
    if iter_count > 10 {
        return input.to_string();
    }
    println!("{}: {}", iter_count, input);
    // let first = input.chars().nth(0);
    // let last = input.chars().nth(input.len() - 1).unwrap();

    let mut result = String::from(input);
    let mut map: HashMap<i32, &str> = HashMap::new();
    map.insert(1, "one");
    map.insert(2, "two");
    map.insert(3, "three");
    map.insert(4, "four");
    map.insert(5, "five");
    map.insert(6, "six");
    map.insert(7, "seven");
    map.insert(8, "eight");
    map.insert(9, "nine");

    // let ks: Vec<i32> = map.keys().cloned().collect();
    let vs: Vec<&str> = map.values().cloned().collect();

    let chars: Vec<char> = input.chars().collect();
    let len = input.len();
    let max_index = len - 1;

    for i in 0..max_index {
        for j in (i + 1)..len {
            let slice: String = chars[i..j].to_vec().iter().collect();
            if vs.contains(&slice.as_str()) {
                let v = &slice;
                let k = *find_key_for_value(&map, v).unwrap();
                result = result.replace(v, &k.to_string());
                parse(&result, iter_count + 1);
            } 
        }
    }
    return result;
}

fn find_key_for_value<'a>(map: &'a HashMap<i32, &'static str>, value: &str) -> Option<&'a i32> {
    map.iter()
        .find_map(|(key, &val)| if val == value { Some(key) } else { None })
}

fn left_number(input: &str) -> i32 {
    for c in input.chars() {
        if c.is_digit(10) {
            if let Some(digit) = c.to_digit(10) {
                return digit as i32;
            }
        }
    }
    0
}

fn right_number(input: &str) -> i32 {
    for c in input.chars().rev() {
        if c.is_digit(10) {
            if let Some(digit) = c.to_digit(10) {
                return digit as i32;
            }
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = process(
            "two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen",
        );
        assert_eq!(result, -69);
    }
}
