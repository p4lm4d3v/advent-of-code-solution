use serde_json::{Map, Value};

fn main() {
    let input: &str = include_str!("./input1.json");
    let output = process(input);
    dbg!(output);
}

fn contains_red_value(obj: &Map<String, Value>) -> bool {
    for (_, value) in obj {
        if let Value::String(s) = value {
            if s == "red" {
                return true;
            }
        }
    }
    false
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
            // Check if the object has any property with the value "red"
            if !contains_red_value(obj) {
                for (_, value) in obj {
                    result.extend(extract_numbers(value));
                }
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
    fn part2_test1() {
        let result = process("[1,{\"c\":\"red\",\"b\":2},3]");
        assert_eq!(result, 4)
    }
    #[test]
    fn part2_test2() {
        let result = process("{\"d\":\"red\",\"e\":[1,2,3,4],\"f\":5}");
        assert_eq!(result, 0)
    }
    #[test]
    fn part2_test3() {
        let result = process("[1,\"red\",5]");
        assert_eq!(result, 6)
    }
}
