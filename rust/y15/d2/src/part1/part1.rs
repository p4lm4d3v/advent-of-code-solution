mod test1;

fn main() {
    let input = include_str!("./input1.txt");
    let output = process(input);
    dbg!("{}", output);
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
    let lines = input.trim().lines().collect::<Vec<_>>();
    let boxes = lines
        .iter()
        .map(|l| {
            let split = l.trim().split("x").collect::<Vec<_>>();
            let l = split[0].parse::<usize>().expect("Can't parse length!");
            let w = split[1].parse::<usize>().expect("Can't parse width!");
            let h = split[2].parse::<usize>().expect("Cant't parse height!");
            Box::new(l, w, h)
        })
        .collect::<Vec<_>>();

    let wrapping_paper = boxes.iter().map(|b| b.area + b.slack).collect::<Vec<_>>();
    wrapping_paper.iter().fold(0, |w1, w2| w1 + w2)
}
