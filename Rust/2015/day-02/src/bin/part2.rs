fn main() {
    let input: &str = include_str!("./input1.txt");
    let output = process(input);
    dbg!(output);
}

#[derive(Debug)]
struct Box {
    wrapping_ribbon: usize,
    bow: usize,
}

impl Box {
    fn new(l: usize, w: usize, h: usize) -> Self {
        Self {
            wrapping_ribbon: 2 * *[l + w, w + h, h + l].iter().min().unwrap(),
            bow: l * w * h,
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

    // boxes
    //     .iter()
    //     .for_each(|b| println!("{}, {}", b.wrapping_ribbon, b.bow));

    let total_ribbon = boxes
        .iter()
        .map(|b| b.wrapping_ribbon + b.bow)
        .collect::<Vec<_>>();
    total_ribbon.iter().fold(0, |w1, w2| w1 + w2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = process("2x3x4");
        assert_eq!(result, 34);
    }
    #[test]
    fn test2() {
        let result = process("1x1x10");
        assert_eq!(result, 14);
    }
}
