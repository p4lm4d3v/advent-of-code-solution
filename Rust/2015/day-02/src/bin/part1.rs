fn main() {
    let input: &str = include_str!("./input1.txt");
    let output = process(input);
    dbg!(output);
}

#[derive(Debug)]
struct Box {
    area: usize,
    slack: usize,
}

impl Box {
    fn new(l: usize, w: usize, h: usize) -> Self {
        Self {
            area: 2 * l * w + 2 * w * h + 2 * h * l,
            slack: *[l * w, w * h, h * l].iter().min().unwrap(),
        }
    }
}

fn process(input: &str) -> usize {
    let lines = input.lines().collect::<Vec<_>>();
    let boxes = lines
        .iter()
        .map(|l| {
            let split = l.split("x").collect::<Vec<_>>();
            let l = split[0].parse::<usize>().unwrap();
            let w = split[1].parse::<usize>().unwrap();
            let h = split[2].parse::<usize>().unwrap();
            Box::new(l, w, h)
        })
        .collect::<Vec<_>>();

    let wrapping_paper = boxes.iter().map(|b| b.area + b.slack).collect::<Vec<_>>();
    wrapping_paper.iter().fold(0, |w1, w2| w1 + w2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = process("2x3x4");
        assert_eq!(result, 58);
    }
    #[test]
    fn test2() {
        let result = process("1x1x10");
        assert_eq!(result, 43);
    }
}
