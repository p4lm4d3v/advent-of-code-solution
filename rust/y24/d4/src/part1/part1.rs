mod test;

fn main() {
    let input = include_str!("./input1.txt");
    let output = process(input);
    dbg!("{}", output);
}
#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
}

impl Direction {
    fn offset(&self) -> (isize, isize) {
        match self {
            Direction::Up => (-1, 0),
            Direction::UpRight => (-1, 1),
            Direction::Right => (0, 1),
            Direction::DownRight => (1, 1),
            Direction::Down => (1, 0),
            Direction::DownLeft => (1, -1),
            Direction::Left => (0, -1),
            Direction::UpLeft => (-1, -1),
        }
    }
    fn all() -> Vec<Self> {
        vec![
            Direction::Up,
            Direction::UpRight,
            Direction::Right,
            Direction::DownRight,
            Direction::Down,
            Direction::DownLeft,
            Direction::Left,
            Direction::UpLeft,
        ]
    }
}

fn process(input: &str) -> i32 {
    let matrix: Vec<Vec<char>> = to_matrix(input);
    let rows = matrix.len();
    let cols = matrix[0].len();

    let mut count = 0;

    for i in 0..rows {
        for j in 0..cols {
            for &direction in &Direction::all() {
                if search_direction(&matrix, "XMAS", i, j, direction) {
                    count += 1;
                }
            }
        }
    }

    count
}

fn to_matrix(input: &str) -> Vec<Vec<char>> {
    if input.is_empty() {
        return vec![vec![]];
    }
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| if ch == ' ' { '.' } else { ch })
                .collect()
        })
        .collect()
}

fn search_direction(
    matrix: &Vec<Vec<char>>,
    word: &str,
    start_row: usize,
    start_col: usize,
    direction: Direction,
) -> bool {
    let rows = matrix.len() as isize;
    let cols = matrix[0].len() as isize;

    let mut i = start_row as isize;
    let mut j = start_col as isize;

    for ch in word.chars() {
        if i < 0 || i >= rows || j < 0 || j >= cols {
            return false;
        }

        let element = matrix[i as usize][j as usize];

        if element != ch {
            return false;
        }

        let (dx, dy) = direction.offset();

        i += dx;
        j += dy;
    }

    true
}
