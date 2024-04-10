use std::{collections::HashMap, time::SystemTime};

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

    let start_nodes = nodes
        .iter()
        .filter(|(k, _)| k.ends_with("A"))
        .collect::<HashMap<_, _>>();

    // for start_node in start_nodes.iter() {
    //     println!("{:?}", start_node);
    // }

    let mut index = 0;
    let mut keys = start_nodes.keys().map(|node| **node).collect::<Vec<_>>();

    keys.sort();
    println!("Step 0: {}", keys.join(", "));

    let _t1 = SystemTime::now();

    while !keys.iter().all(|k| k.ends_with("Z")) {
        let dir = directions[index % directions.len()];

        keys = match dir {
            'L' => keys.iter().map(|k| nodes[k].0).collect::<Vec<&str>>(),
            'R' => keys.iter().map(|k| nodes[k].1).collect::<Vec<&str>>(),
            _ => keys,
        };

        println!("Step {}: {}", index + 1, keys.join(", "));

        // keys = keys
        //     .iter()
        //     .filter(|k| !k.ends_with("Z"))
        //     .map(|k| *k)
        //     .collect::<Vec<_>>();

        index += 1;
    }
    let _t2 = SystemTime::now();
    println!("{} s", _t2.duration_since(_t1).unwrap().as_secs_f64());

    index
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = process(
            "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)",
        );
        assert_eq!(result, 6);
    }
    #[test]
    fn test2() {
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
    fn test3() {
        let result = process(
            "LLR

        AAA = (BBB, BBB)
        BBB = (AAA, ZZZ)
        ZZZ = (ZZZ, ZZZ)",
        );
        assert_eq!(result, 6);
    }
}
