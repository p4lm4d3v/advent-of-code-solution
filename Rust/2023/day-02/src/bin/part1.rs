use std::{collections::HashMap, fs::File, io::Write};

use serde::Serialize;

fn main() {
    let input: &str = include_str!("./input1.txt");
    let output: i32 = process(input);
    dbg!(output);
}
#[derive(Serialize, Debug)]
struct Cube {
    color: String,
}

impl Cube {
    fn new(color: &str) -> Self {
        Self {
            color: color.to_string(),
        }
    }
}

#[derive(Serialize, Debug)]
struct Take {
    cubes: HashMap<i32, Cube>,
}

impl Take {
    fn new(take: &str) -> Self {
        let mut cubes: HashMap<i32, Cube> = HashMap::new();
        let cubes_strs: Vec<&str> = take.split(", ").collect();

        for c in cubes_strs.iter() {
            let split: Vec<&str> = c.trim().split(" ").collect();
            let amount: i32 = split.get(0).unwrap().parse().unwrap();
            let color = split.get(1).unwrap();

            let cube = Cube::new(color);
            cubes.insert(amount, cube);
        }

        Self { cubes: cubes }
    }
}


#[derive(Debug, serde::Serialize)]
struct Game {
    id: i32,
    takes: Vec<Take>,
}

impl Game {
    fn new(line: &str) -> Self {
        let split: Vec<&str> = line.split(":").collect();
        let left = split.get(0).unwrap().trim();
        let right = split.get(1).unwrap().trim();

        let id: i32 = left.replace("Game", "").trim().parse().unwrap();

        let mut takes: Vec<Take> = vec![];
        let takes_strs: Vec<&str> = right.split(";").collect();

        for t in takes_strs.iter() {
            let take = Take::new(t.trim());
            takes.push(take);
        }

        Self {
            id: id,
            takes: takes,
        }
    }

    fn can_exist(&self, red: i32, green: i32, blue: i32) -> bool {
        for t in &self.takes {
            for (amount, cube) in &t.cubes {
                let result = match cube.color.as_str() {
                    "red" => *amount <= red,
                    "green" => *amount <= green,
                    "blue" => *amount <= blue,
                    _ => false
                };
                if !result {
                    return false;
                }
            }
        }
        true
    }
}

fn process(input: &str) -> i32 {
    let lines: Vec<&str> = input.lines().collect();
    let mut game_ids: Vec<i32> = vec![]; 
    
    for l in lines {
        let line = l.trim();
        let game = Game::new(line);
        if game.can_exist(12, 13, 14) {
            game_ids.push(game.id);
        }
        let json = serde_json::to_string(&game).unwrap();
        let mut file = File::create("test\\game".to_string() + &game.id.to_string() + ".json", ).unwrap();
        let _ = file.write(json.as_bytes());
    }
    game_ids.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result: i32 = process(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        );
        assert_eq!(result, 8);
    }
}
