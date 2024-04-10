use std::vec;

fn main() {
    let input: &str = include_str!("./input1.txt");
    let output = process(input);
    dbg!(output);
}

static UP_LEFT: &str = "┌";
static UP_RIGHT: &str = "┐";
static DOWN_LEFT: &str = "└";
static DOWN_RIGHT: &str = "┘";
static HORIZONTAL: &str = "─";
static VERTICAL: &str = "│";
static START: &str = "#";

fn clean_map(input: &str) -> String {
    input
        // .replace(".", EMPTY.to_string().as_str())
        .replace("S", START)
        .replace("F", UP_LEFT)
        .replace("7", UP_RIGHT)
        .replace("L", DOWN_LEFT)
        .replace("J", DOWN_RIGHT)
        .replace("-", HORIZONTAL)
        .replace("|", VERTICAL)
}

fn lines_to_matrix(input: &str) -> Vec<Vec<&str>> {
    input
        .lines()
        .map(|l| {
            l.trim()
                .split("")
                .filter(|l| !l.is_empty())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<Vec<_>>>()
}

fn dfs_nodes(
    matrix: &Vec<Vec<&str>>,
    visited: &mut Vec<Vec<bool>>,
    nodes: &mut Vec<Node>,
    current: (usize, usize),
    distance: usize,
) {
    let (row, col) = current;
    let rows = matrix.len();
    let cols = matrix[0].len();

    if row >= rows || col >= cols || visited[row][col] || matrix[row][col] == "." {
        return;
    }

    visited[row][col] = true;
    let new_node = Node {
        pos: (row, col),
        distance: Some(distance),
    };
    nodes.push(new_node);

    // Define the possible moves (up, right, down, left)
    let moves = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];

    for (dr, dc) in moves {
        let new_row = (row as isize + dr) as usize;
        let new_col = (col as isize + dc) as usize;

        if new_row < rows && new_col < cols && !visited[new_row][new_col] {
            dfs_nodes(matrix, visited, nodes, (new_row, new_col), distance + 1);
        }
    }
}

fn get_nodes(matrix: &Vec<Vec<&str>>, start: (usize, usize)) -> Vec<Node> {
    let mut visited = vec![vec![false; matrix[0].len()]; matrix.len()];
    let mut nodes = Vec::new();

    dfs_nodes(matrix, &mut visited, &mut nodes, start, 0);

    nodes
}

fn get_target_node(matrix: &Vec<Vec<&str>>, target: &str) -> Node {
    let indices = matrix
        .iter()
        .enumerate()
        .find_map(|(i, row)| row.iter().position(|&x| x == target).map(|j| (i, j)))
        .unwrap();
    Node::new(None, indices)
}

fn weighted_map(input: &str) -> (Vec<Vec<usize>>, String) {
    let matrix = lines_to_matrix(input);

    println!("Map of pipes: ");
    for line in matrix.iter() {
        for s in line {
            print!("{} ", s);
        }
        println!();
    }

    let start = get_target_node(&matrix, START);

    let nodes: Vec<Node> = get_nodes(&matrix, start.pos);

    let mut vec: Vec<Vec<String>> = vec![];
    let len = matrix.len();
    for _ in 0..len {
        let mut v = vec![];
        for _ in 0..len {
            v.push(String::from(""));
        }
        vec.push(v);
    }

    for i in 0..len {
        for j in 0..len {
            let ij = (i, j);

            let pos = nodes
                .iter()
                .filter(|node| node.pos == ij)
                .collect::<Vec<_>>();

            if !pos.is_empty() {
                for p in pos {
                    let dist = p.distance.unwrap().to_string();
                    vec[ij.0][ij.1] = dist;
                }
            } else {
                vec[ij.0][ij.1] = "0".to_string();
            }
        }
    }

    let mut output = "".to_string();

    for i in 0..len {
        for j in 0..len {
            output = output.to_string() + &vec[i][j];
        }
        output += "\n";
    }

    let v = vec
        .iter()
        .map(|x| {
            x.iter()
                .map(|y| y.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<_>>();

    (v, output.to_string())
}

fn process(input: &str) -> usize {
    let clean_input = clean_map(input);
    let (output, weighted_input) = weighted_map(&clean_input);

    println!("{}", weighted_input);

    let mut res = vec![];
    for i in 0..output.len() {
        for j in 0..output[0].len() {
            res.push(output[i][j]);
        }
    }
    *res.iter().max().unwrap()
}

#[derive(Debug, Clone)]
struct Node {
    distance: Option<usize>,
    pos: (usize, usize),
}

impl Node {
    fn new(distance: Option<usize>, pos: (usize, usize)) -> Self {
        Self {
            distance: distance,
            pos: pos,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test1() {
        let result = process(
            ".....
.S-7.
.|.|.
.L-J.
.....",
        );
        assert_eq!(result, 4);
    }
    #[test]
    fn part1_test2() {
        let result = process(
            "..F7.
.FJ|.
SJ.L7
|F--J
LJ...",
        );
        assert_eq!(result, 8);
    }
}
