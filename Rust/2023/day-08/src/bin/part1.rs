use std::collections::HashMap;

fn main() {
    let input: &str = include_str!("./input1.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> usize {
    let lines = input
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .collect::<Vec<&str>>();

    let directions = lines.first().unwrap().chars().collect::<Vec<char>>();

    let mut nodes: HashMap<&str, (&str, &str)> = HashMap::new();

    for line in lines[1..].iter() {
        let line = *line;

        let split = line.split(" ").collect::<Vec<&str>>();

        let name = split[0];
        let left = split[2].trim_matches(|c| c == '(' || c == ',');
        let right = split[3].trim_matches(|c| c == ',' || c == ')');

        nodes.insert(name, (left, right));
    }

    let mut key = "AAA";
    let mut index = 0;

    let mut keys: Vec<&str> = Vec::new();

    while key != "ZZZ" {
        let dir = directions[index % directions.len()];
        keys.push(key);
        key = match dir {
            'L' => nodes[key].0,
            'R' => nodes[key].1,
            _ => key,
        };

        index += 1;
    }

    index
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = process(
            "RL

        AAA = (BBB, CCC)
        BBB = (DDD, EEE)
        CCC = (ZZZ, GGG)
        DDD = (DDD, DDD)
        EEE = (EEE, EEE)
        GGG = (GGG, GGG)
        ZZZ = (ZZZ, ZZZ)",
        );
        assert_eq!(result, 2);
    }
    #[test]
    fn test2() {
        let result = process(
            "LLR

        AAA = (BBB, BBB)
        BBB = (AAA, ZZZ)
        ZZZ = (ZZZ, ZZZ)",
        );
        assert_eq!(result, 6);
    }
}
