use std::collections::HashMap;

fn main() {
    let input: &str = include_str!("./input2.txt");
    let output: i32 = process(input);
    dbg!(output);
}
#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
struct Game {
    takes: Vec<Take>,
}

impl Game {
    fn new(line: &str) -> Self {
        let split: Vec<&str> = line.split(":").collect();
        let right = split.get(1).unwrap().trim();

        let mut takes: Vec<Take> = vec![];
        let takes_strs: Vec<&str> = right.split(";").collect();

        for t in takes_strs.iter() {
            let take = Take::new(t.trim());
            takes.push(take);
        }

        Self {
            takes: takes,
        }
    }

    fn min_amounts(&self) -> (i32, i32, i32) {
        let mut reds: Vec<i32> = vec![];
        let mut greens: Vec<i32> = vec![];
        let mut blues: Vec<i32> = vec![];

        for t in &self.takes {
            for (amount, cube) in &t.cubes {
                match cube.color.as_str() {
                    "red" => reds.push(*amount),
                    "green" => greens.push(*amount),
                    "blue" => blues.push(*amount),
                    _ => todo!(),
                };
            }
        }
        (
            *reds.iter().clone().max().unwrap(),
            *greens.iter().clone().max().unwrap(),
            *blues.iter().clone().max().unwrap()
        )
    }
}

fn process(input: &str) -> i32 {
    let lines: Vec<&str> = input.lines().collect();
    let mut powers: Vec<i32> = vec![];

    for l in lines {
        let line = l.trim();
        let game = Game::new(line);
        let (r, g, b) = game.min_amounts();
        let power = r * g * b;
        powers.push(power);
        println!("{} * {} * {} = {}", r, g, b, power);
    }
    powers.iter().sum()
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
        assert_eq!(result, 2286);
    }
}
