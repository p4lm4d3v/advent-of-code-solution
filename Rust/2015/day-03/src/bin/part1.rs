use std::collections::HashSet;

fn main() {
    let input: &str = include_str!("./input1.txt");
    let output = process(input);
    dbg!(output);
}

struct Path {
    up_down: i32,
    right_left: i32,
}

impl Path {
    fn new() -> Self {
        Self {
            up_down: 0,
            right_left: 0,
        }
    }
    fn get_state(&self) -> String {
        format!("{}:{}", self.up_down, self.right_left)
    }
}

fn process(input: &str) -> usize {
    let directions = input.split("").filter(|dir| *dir != "").collect::<Vec<_>>();

    let mut path = Path::new();

    let mut distinct_houses: HashSet<String> = HashSet::new();
    distinct_houses.insert("0:0".to_string());

    for dir in directions.iter() {
        let dir = *dir;
        match dir {
            "^" => path.up_down += 1,
            ">" => path.right_left += 1,
            "v" => path.up_down -= 1,
            "<" => path.right_left -= 1,
            _ => (),
        }

        distinct_houses.insert(path.get_state());
    }

    distinct_houses.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test1() {
        let result = process(">");
        assert_eq!(result, 2);
    }
    #[test]
    fn part1_test2() {
        let result = process("^>v<");
        assert_eq!(result, 4);
    }
    #[test]
    fn part1_test3() {
        let result = process("^v^v^v^v^v");
        assert_eq!(result, 2);
    }
}
