use serde_json::Value;

fn main() {
    let input: &str = include_str!("./input1.json");
    let output = process(input);
    dbg!(output);
}

fn extract_numbers(json_data: &Value) -> Vec<i32> {
    let mut result = Vec::new();

    match json_data {
        Value::Array(arr) => {
            for item in arr {
                result.extend(extract_numbers(item));
            }
        }
        Value::Object(obj) => {
            for (_, value) in obj {
                result.extend(extract_numbers(value));
            }
        }
        Value::Number(number) => {
            if let Some(num) = number.as_i64() {
                result.push(num as i32);
            }
        }
        _ => {}
    }

    result
}

fn get_numbers(input: &str) -> Vec<i32> {
    let json: Value = serde_json::from_str(input).expect("JSON was not well-formatted");
    extract_numbers(&json)
}

fn process(input: &str) -> usize {
    let numbers = get_numbers(input);
    numbers.iter().fold(0, |a, b| a + b) as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test1() {
        let result = process("[1,2,3]");
        assert_eq!(result, 6);
    }
    #[test]
    fn part1_test2() {
        let result = process("{\"a\":2,\"b\":4}");
        assert_eq!(result, 6);
    }
    #[test]
    fn part1_test3() {
        let result = process("[[[3]]]");
        assert_eq!(result, 3);
    }
    #[test]
    fn part1_test4() {
        let result = process("{\"a\":{\"b\":4},\"c\":-1}");
        assert_eq!(result, 3);
    }
    #[test]
    fn part1_test5() {
        let result = process("{\"a\":[-1,1]}");
        assert_eq!(result, 0);
    }
    #[test]
    fn part1_test6() {
        let result = process("[-1,{\"a\":1}]");
        assert_eq!(result, 0);
    }
    #[test]
    fn part1_test7() {
        let result = process("[]");
        assert_eq!(result, 0);
    }

    #[test]
    fn part1_test8() {
        let result = process("{}");
        assert_eq!(result, 0);
    }
}
