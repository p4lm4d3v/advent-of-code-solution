fn main() {
    let input: &str = include_str!("./input1.txt");
    let output = process(input);
    dbg!(output);
}

#[derive(Debug)]
struct Race {
    time: u32,
    max_distance: u32,
}

impl Race {
    fn new(time: u32, max_distance: u32) -> Self {
        Self {
            time,
            max_distance,
        }
    }

    fn get_winnings(&self) -> Vec<(u32, u32)> {
        let mut result: Vec<(u32, u32)> = vec![];

        for i in 0..=self.time {
            let remaining_time = self.time - i;
            let distance = remaining_time * i;
            result.push((i, distance));
        }

        result
            .iter()
            .filter(|d| d.1 > self.max_distance)
            .map(|d| *d)
            .collect::<Vec<(u32, u32)>>()
    }
}

fn process(input: &str) -> u32 {
    let mut games: Vec<Race> = vec![];

    let lines = input.lines().map(|l| l.trim()).collect::<Vec<&str>>();

    let lines1 = lines.clone();
    let lines2 = lines.clone();

    let times = lines1.first().unwrap().replace("Time:", "");
    let times = times
        .trim()
        .split(" ")
        .filter(|t| !t.is_empty())
        .map(|t| t.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let distances = lines2.last().unwrap().replace("Distance: ", "");
    let distances = distances
        .trim()
        .split(" ")
        .filter(|d| !d.is_empty())
        .map(|d| d.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    for i in 0..times.len() {
        let game = Race::new(*times.get(i).unwrap(), *distances.get(i).unwrap());
        games.push(game);
    }

    let mut numbers_of_winnings:Vec<u32> = vec![];
    for (i, game) in games.iter().enumerate() {
        let number_of_wins = game.get_winnings().len();
        numbers_of_winnings.push(number_of_wins as u32);
        println!("{:?} #{}, number of wins: {}", game, i + 1, number_of_wins);
    }

    numbers_of_winnings
    .iter()
    .fold(1, |x, y| x * y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = process(
            "Time:      7  15   30
        Distance:  9  40  200",
        );
        assert_eq!(result, 4 * 8 * 9);
    }
}
