use array_tool::vec::Intersect;

fn main() {
    let input: &str = include_str!("./input1.txt");
    let output = process(input);
    dbg!(output);
}

struct Card {
    winning_numbers: Vec<u32>,
    card_numbers: Vec<u32>,
}

impl Card {
    fn new(input: &str) -> Self {
        let parts: Vec<&str> = input.trim().split('|').map(|s| s.trim()).collect();

        let winning_numbers: Vec<u32> = parts[0]
            .split_whitespace()
            .filter_map(|num| num.parse().ok())
            .collect();

        let card_numbers: Vec<u32> = parts[1]
            .split_whitespace()
            .filter_map(|num| num.parse().ok())
            .collect();
        
        Self {
            winning_numbers: winning_numbers,
            card_numbers: card_numbers,
        }
    }
    fn intersection(&self) -> Vec<u32> {
        let v1 = self.winning_numbers.clone();
        let v2 = self.card_numbers.clone();
        v1.intersect(v2)
    }
    fn worth(&self) -> u32 {
        let hits = self.intersection().len() as u32;

        if hits != 0 {
            return 2_u32.pow(hits - 1);
        }
        0
    }
}

fn process(input: &str) -> u32 {
    let lines: Vec<&str> = input.lines().collect();
    let mut sum = 0;
    for (i, l) in lines.iter().enumerate() {
        let line = l.trim_start().trim_end();
        let card = Card::new(line);
        // println!("Game {} has worth {}", i, card.worth());
        sum += card.worth();
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = process(
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        );
        assert_eq!(result, 13);
    }
}
