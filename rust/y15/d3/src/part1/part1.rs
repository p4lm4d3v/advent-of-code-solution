use std::collections::HashSet;

mod test1;

fn main() {
    let input = include_str!("./input1.txt");
    let output = process(input);
    dbg!("{}", output);
}
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

    let mut path = Path::new();

    let mut distinct_houses: HashSet<String> = HashSet::new();
    distinct_houses.insert("0:0".to_string());

    for dir in directions.iter() {
        let dir = *dir;
        match dir {
            "^" => path.vertical += 1,
            ">" => path.horizontal += 1,
            "v" => path.vertical -= 1,
            "<" => path.horizontal -= 1,
            _ => (),
        }

        distinct_houses.insert(path.get_state());
    }

    distinct_houses.len()
}
