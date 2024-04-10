use std::{collections::VecDeque, vec};

fn main() {
    let input: &str = include_str!("./input1.txt");
    let output = process(input);
    dbg!(output);
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
struct Galaxy {
    name: char,
    pos: Node,
}

impl Galaxy {
    fn new(name: char, pos: Node) -> Self {
        Self {
            name: name,
            pos: pos,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct Node {
    i: usize,
    j: usize,
}

impl Node {
    fn new(i: usize, j: usize) -> Self {
        Self { i: i, j: j }
    }
}

fn expand_rows(input: &str) -> String {
    let mut rows = input.lines().collect::<Vec<_>>();

    let empty_rows = rows
        .iter()
        .enumerate()
        .map(|(i, r)| (i, *r))
        .collect::<Vec<(usize, &str)>>()
        .iter()
        .filter(|(_, s)| !s.chars().any(|c| c == '#'))
        .map(|t| *t)
        .collect::<Vec<(usize, &str)>>();

    let mut already_added: usize = 0;
    empty_rows.iter().for_each(|t| {
        rows.insert(t.0 + already_added, t.1);
        already_added += 1;
    });

    let mut output: Vec<&str> = vec![];

    for row in rows {
        output.push(row);
    }

    let output = output.join("\n");
    output
}

fn expand_columns(input: &str) -> String {
    let mut rows = input
        .lines()
        .map(|line| line.to_string())
        .collect::<Vec<_>>();
    let num_columns = rows.iter().map(|row| row.len()).max().unwrap_or(0);

    let empty_columns: Vec<usize> = (0..num_columns)
        .filter(|&j| {
            !rows
                .iter()
                .any(|row| row.chars().nth(j).unwrap_or(' ') == '#')
        })
        .collect();

    let mut counter = 0;
    for &j in &empty_columns {
        rows.iter_mut().for_each(|row| {
            if j < row.len() {
                row.insert(j + counter, '.');
            } else {
                row.push('.');
            }
        });
        counter += 1;
    }

    let output = rows.into_iter().collect::<Vec<_>>();
    output.join("\n")
}

fn expand_map(input: &str) -> String {
    let expanded_by_rows = expand_rows(input);
    let expanded_by_cols = expand_columns(&expanded_by_rows);
    expanded_by_cols
}

fn numbered_map(input: &str) -> String {
    let lines = input.lines();
    let width = lines.clone().nth(0).unwrap().len();
    let height = lines.clone().count();

    let mut counter = 1;
    let mut matrix = vec![vec![' '; width]; height];

    for (i, row) in lines.enumerate() {
        for (j, char) in row.chars().enumerate() {
            if char == '#' {
                matrix[i].insert(j, counter.to_string().chars().next().unwrap());
                counter += 1;
            } else {
                matrix[i].insert(j, '.');
            }
        }
    }

    let output = matrix
        .iter()
        .map(|v| {
            v.iter()
                .filter(|c| **c != ' ')
                .map(|i| i.to_string())
                .collect::<String>()
        })
        .collect::<Vec<String>>();

    let output = output.join("\n");
    let output = output.replace("0", ".");
    output
}

fn get_galaxies(numbered_map: &str) -> Vec<Galaxy> {
    let mut galaxies: Vec<Galaxy> = vec![];

    numbered_map.lines().enumerate().for_each(|(i, l)| {
        l.chars().enumerate().for_each(|(j, c)| {
            if c != '.' {
                let galaxy = Galaxy::new(c, Node::new(i, j));
                galaxies.push(galaxy);
            }
        });
    });

    galaxies
}

fn get_galaxy_pairs(galaxies: &Vec<Galaxy>) -> Vec<(Galaxy, Galaxy)> {
    let mut pairs: Vec<(_, _)> = vec![];
    for galaxy1 in galaxies.iter() {
        for galaxy2 in galaxies.iter() {
            if galaxy1.name != galaxy2.name {
                if !pairs.contains(&(galaxy1, galaxy2)) && !pairs.contains(&(galaxy2, galaxy1)) {
                    pairs.push((galaxy1, galaxy2));
                }
            }
        }
    }
    pairs
        .iter()
        .map(|(g1, g2)| (**g1, **g2))
        .collect::<Vec<_>>()
}

fn djikstra(start: Node, end: Node) -> Vec<Node> {
    vec![]
}

fn process(input: &str) -> usize {
    let lines = input.lines().collect::<Vec<_>>();
    let width = lines.first().unwrap().len();
    let height = lines.len();
    let expanded_map = expand_map(input);
    let numbered_map = numbered_map(&expanded_map);

    let galaxies: Vec<Galaxy> = get_galaxies(&numbered_map);
    let galaxi_pairs = get_galaxy_pairs(&galaxies);

    galaxi_pairs.iter().enumerate().for_each(|(i, (g1, g2))| {
        let steps = djikstra(g1.pos, g2.pos);
        println!(
            "#{} {} to {} => {}",
            i + 1,
            g1.name,
            g2.name,
            steps.iter().len(),
        )
    });
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = process(
            "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....",
        );
        assert_eq!(result, 374);
    }
}
