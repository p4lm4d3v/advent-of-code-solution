use std::collections::HashSet;

fn main() {
    let input: &str = include_str!("./input1.txt");
    let output = process(input);
    dbg!(output);
}

#[derive(Debug, Clone, Copy)]
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

    let mut santa_path = Path::new();
    let mut robo_santa_path = Path::new();

    let mut distinct_houses: HashSet<String> = HashSet::new();
    distinct_houses.insert("0:0".to_string());

    for (i, dir) in directions.iter().enumerate() {
        let i = i + 2;

        let dir = *dir;
        if i % 2 == 0 {
            match dir {
                "^" => santa_path.up_down += 1,
                ">" => santa_path.right_left += 1,
                "v" => santa_path.up_down -= 1,
                "<" => santa_path.right_left -= 1,
                _ => (),
            }
        } else {
            match dir {
                "^" => robo_santa_path.up_down += 1,
                ">" => robo_santa_path.right_left += 1,
                "v" => robo_santa_path.up_down -= 1,
                "<" => robo_santa_path.right_left -= 1,
                _ => (),
            }
        };

        distinct_houses.insert(santa_path.get_state());
        distinct_houses.insert(robo_santa_path.get_state());
        // println!(
        //     "{} => N: {}, R: {} => {:?}",
        //     i,
        //     santa_path.get_state(),
        //     robo_santa_path.get_state(),
        //     distinct_houses
        // );
    }

    distinct_houses.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part2_test1() {
        let result = process("^v");
        assert_eq!(result, 3);
    }
    #[test]
    fn part2_test2() {
        let result = process("^>v<");
        assert_eq!(result, 3);
    }
    #[test]
    fn part2_test3() {
        let result = process("^v^v^v^v^v");
        assert_eq!(result, 11);
    }
}
