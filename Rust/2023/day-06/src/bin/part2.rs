fn main() {
    let input: &str = include_str!("./input1.txt");
    let output = process(input);
    dbg!(output);
}

#[derive(Debug, Clone, Copy)]
struct Race {
    time: f32,
    max_distance: f32,
}

impl Race {
    fn new(time: f32, max_distance: f32) -> Self {
        Self { time, max_distance }
    }

    fn get_winnings(&self) -> Vec<(f32, f32)> {
        let mut res: Vec<(f32, f32)> = vec![];

        let t: usize = (self.time * 1000.0) as usize;

        for i in 0..=t {
            let remaining_time = self.time - (i as f32 / 1000.0);
            let distance = remaining_time * i as f32;
            res.push((i as f32, distance));
            // println!("{}*{}={}", i, remaining_time, distance);
        }

        res.iter()
            .filter(|d| d.1 > self.max_distance)
            .map(|d| *d)
            .collect::<Vec<(f32, f32)>>()
    }
}

fn process(input: &str) -> usize {
    let lines = input.lines().map(|l| l.trim()).collect::<Vec<&str>>();

    let lines1 = lines.clone();
    let lines2 = lines.clone();

    let times = lines1.first().unwrap().replace("Time:", "");
    let times = times
        .trim()
        .split(" ")
        .filter(|t| !t.is_empty())
        .map(|t| t.to_string())
        .collect::<Vec<String>>()
        .join("")
        .parse::<usize>()
        .unwrap();

    let distances = lines2.last().unwrap().replace("Distance: ", "");
    let distances = distances
        .trim()
        .split(" ")
        .filter(|d| !d.is_empty())
        .map(|d| d.to_string())
        .collect::<Vec<String>>()
        .join("")
        .parse::<usize>()
        .unwrap();

    let race = Race::new(times as f32 / 1000.0, distances as f32 / 1000.0);
    let mut winnings = race.get_winnings();

    // for ele in winnings.iter() {
    //     println!("{}", ele.0);
    // }

    winnings.sort_by(|a,b| a.partial_cmp(b).unwrap());
    print!("{}", winnings.first().unwrap().1);
    print!("{}", winnings.last().unwrap().1);
    
    println!("{}", winnings.len());
    winnings.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = process(
            "Time:      7 15 30
            Distance:  9 40 200",
        );
        assert_eq!(result, 71503);
    }
}
