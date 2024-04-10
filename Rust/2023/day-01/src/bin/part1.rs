fn main() {
    let input: &str = include_str!("./input1.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> i32 {
    let lines: Vec<&str> = input.lines().collect();
    let mut numbers: Vec<i32> = vec![];
    for l in lines {
        let line = l.trim();
        let left_number = left_number(line);
        let right_number = right_number(line);
        let str = left_number.to_string() + &right_number.to_string();
        let num: i32 = str.parse().unwrap();
        numbers.push(num);
    }
    numbers.iter().sum()
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
            "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet",
        );
        assert_eq!(result, 142);
    }
}
