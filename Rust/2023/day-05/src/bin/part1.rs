use std::collections::HashMap;

fn main() {
    let input: &str = include_str!("./input1.txt");
    let output = process(input);
    dbg!(output);
}

#[derive(Debug)]
struct CustomMap {
    tag: String,
    map: HashMap<u32, u32>,
}

impl CustomMap {
    fn from(tag: &str, slice: &[(u32, u32)]) -> Self {
        let mut map = HashMap::new();

        for (key, value) in slice.iter() {
            map.insert(*key, *value);
        }

        CustomMap::new(tag, map)
    }
}

impl Default for CustomMap {
    fn default() -> Self {
        Self {
            tag: "default".to_string(),
            map: HashMap::new(),
        }
    }
}

impl CustomMap {
    // Constructor
    fn new(tag: &str, map: HashMap<u32,u32>) -> Self {
        Self {
            tag: String::from(tag),
            map: HashMap::new(),
        }
    }
    // Example method to insert a key-value pair into the HashMap
    fn insert(&mut self, key: u32, value: u32) {
        self.map.insert(key, value);
    }
    // Example method to retrieve a value from the HashMap
    fn get(&self, key: &u32) -> Option<&u32> {
        self.map.get(key)
    }
}

fn process(input: &str) -> u32 {
    let lines = input.lines()
    .map(|l| l.trim())
    .filter(|l| !l.is_empty())
    .collect::<Vec<&str>>();

    let seeds: Vec<u32> = lines.first().unwrap()
    .replace("seeds: ", "")
    .split(" ").map(|s| s.parse::<u32>().unwrap()).collect(); 
    
    let temp = lines.get(1).unwrap()
    .replace("map:", "");
    let temp = temp.trim();
    
    let seed_soil_map = CustomMap::default();
    let map: HashMap<u32, u32> = HashMap::new();

    

    // let temp = seeds.iter()
    // .map(|seed| seed.to_string())
    // .collect::<Vec<String>>();
    println!("{:?}", seed_soil_map);


    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result: u32 = process(
            "seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48
        
        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15
        
        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4
        
        water-to-light map:
        88 18 7
        18 25 70
        
        light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13
        
        temperature-to-humidity map:
        0 69 1
        1 0 69
        
        humidity-to-location map:
        60 56 37
        56 93 4",
        );
        assert_eq!(result, 35);
    }
}
