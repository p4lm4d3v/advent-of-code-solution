use std::{
    borrow::{Borrow, BorrowMut},
    collections::VecDeque,
    vec,
};

use colored::Colorize;

fn main() {
    let input: &str = include_str!("./input1.txt");
    let output = process(input);
    dbg!(output);
}

#[derive(Debug)]
enum WhatToDo {
    TurnOn,
    TurnOff,
    Toggle,
    Default,
}

struct LightsMatrix {
    data: Vec<Vec<usize>>,
    size: usize,
}

impl LightsMatrix {
    fn new(size: usize) -> Self {
        let d = (0..size).map(|_| 0).collect::<Vec<_>>();
        let data = Vec::from_iter(
            (0..size)
                .map(|_| d.iter().map(|f| *f).clone().collect::<Vec<_>>())
                .collect::<Vec<_>>(),
        );
        Self {
            data: data,
            size: size,
        }
    }
    fn total(&self) -> usize {
        let mut sum = 0;
        for row in self.data.iter() {
            for element in row {
                sum += element;
            }
        }
        sum
    }
    fn print(&self) {
        for row in self.data.iter() {
            for element in row {
                print!("{} ", element);
            }
            println!();
        }
    }
    fn set_square_range(&mut self, c1: (usize, usize), c2: (usize, usize), what_to_do: WhatToDo) {
        let bul = c1.0 < self.size && c1.1 < self.size && c2.0 < self.size && c2.1 < self.size;
        if bul {
            for x in c1.0..=c2.0 {
                for y in c1.1..=c2.1 {
                    let x = x as usize;
                    let y = y as usize;
                    match what_to_do {
                        WhatToDo::TurnOn => self.data[x][y] += 1,
                        WhatToDo::TurnOff => {
                            if self.data[x][y] > 0 {
                                self.data[x][y] -= 1;
                            }
                        }
                        WhatToDo::Toggle => self.data[x][y] += 2,
                        _ => (),
                    }
                }
            }
        }
    }
}

fn process(input: &str) -> usize {
    let lines = input.lines().map(|l| l.trim()).collect::<Vec<_>>();
    let mut matrix = LightsMatrix::new(1000);

    for line in lines {
        let mut what_to_do: WhatToDo = WhatToDo::Default;
        let mut l = line.to_string();

        if line.contains("turn on") {
            what_to_do = WhatToDo::TurnOn;
            l = line.replace("turn on ", "");
        } else if line.contains("turn of") {
            what_to_do = WhatToDo::TurnOff;
            l = line.replace("turn off ", "");
        } else if line.contains("toggle") {
            what_to_do = WhatToDo::Toggle;
            l = line.replace("toggle ", "");
        }
        let split = l.split(" through ").collect::<Vec<_>>();
        let first = split.first().unwrap().split(",").collect::<Vec<_>>();
        let last = split.last().unwrap().split(",").collect::<Vec<_>>();

        let first_coords = (
            first.first().unwrap().parse::<usize>().unwrap(),
            first.last().unwrap().parse::<usize>().unwrap(),
        );
        let last_coords = (
            last.first().unwrap().parse::<usize>().unwrap(),
            last.last().unwrap().parse::<usize>().unwrap(),
        );

        matrix.set_square_range(first_coords, last_coords, what_to_do);
    }
    matrix.total()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = process("turn on 0,0 through 0,0");
        assert_eq!(result, 1);
    }
    #[test]
    fn test2() {
        let result = process("toggle 0,0 through 999,999");
        assert_eq!(result, 2_000_000)
    }
}
