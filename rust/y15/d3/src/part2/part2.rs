use std::collections::HashSet;

mod test2;

fn main() {
    let input = include_str!("./input2.txt");
    let output = process(input);
    dbg!("{}", output);
}

#[derive(Debug, Clone, Copy)]
struct Path {
    vertical: i32,
    horizontal: i32,
}

impl Path {
    fn new() -> Self {
        Self {
            vertical: 0,
            horizontal: 0,
        }
    }
    fn get_state(&self) -> String {
        format!("{}:{}", self.vertical, self.horizontal)
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
                "^" => santa_path.vertical += 1,
                ">" => santa_path.horizontal += 1,
                "v" => santa_path.vertical -= 1,
                "<" => santa_path.horizontal -= 1,
                _ => (),
            }
        } else {
            match dir {
                "^" => robo_santa_path.vertical += 1,
                ">" => robo_santa_path.horizontal += 1,
                "v" => robo_santa_path.vertical -= 1,
                "<" => robo_santa_path.horizontal -= 1,
                _ => (),
            }
        };

        distinct_houses.insert(santa_path.get_state());
        distinct_houses.insert(robo_santa_path.get_state());
    }

    distinct_houses.len()
}
