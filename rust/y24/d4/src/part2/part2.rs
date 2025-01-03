use std::fmt::{Display, Formatter, Result};

mod test;

fn main() {
    let input = include_str!("./input2.txt");
    let output = process(input);
    dbg!("{}", output);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Dir {
    U,  // up
    Ur, // up-right
    R,  // right
    Dr, // down-right
    D,  // down
    Dl, // down-left
    L,  // left
    Ul, // up-left
}

/*
  ul u ur
   l   r
  dl d dr
*/

impl Dir {
    fn offset(&self) -> (isize, isize) {
        match self {
            Dir::U => (-1, 0),
            Dir::Ur => (-1, 1),
            Dir::R => (0, 1),
            Dir::Dr => (1, 1),
            Dir::D => (1, 0),
            Dir::Dl => (1, -1),
            Dir::L => (0, -1),
            Dir::Ul => (-1, -1),
        }
    }
    fn all() -> Vec<Self> {
        vec![
            Dir::U,
            Dir::Ur,
            Dir::R,
            Dir::Dr,
            Dir::D,
            Dir::Dl,
            Dir::L,
            Dir::Ul,
        ]
    }
    fn is_cardinal(&self) -> bool {
        match self {
            Dir::U | Dir::R | Dir::D | Dir::L => true,
            _ => false,
        }
    }
}

impl Display for Dir {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let display = match self {
            Dir::U => "up",
            Dir::Ur => "up-right",
            Dir::R => "right",
            Dir::Dr => "down-right",
            Dir::D => "down",
            Dir::Dl => "down-left",
            Dir::L => "left",
            Dir::Ul => "up-left",
        };
        write!(f, "{}", display)
    }
}

fn process(input: &str) -> i32 {
    let matrix: Vec<Vec<char>> = to_matrix(input);
    let rows = matrix.len();
    let cols = matrix[0].len();

    let mut occurrences = Vec::new();

    for i in 0..rows {
        for j in 0..cols {
            for &direction in &Dir::all() {
                if search_direction(&matrix, "MAS", i, j, direction) {
                    if !direction.is_cardinal() {
                        occurrences.push((i, j, direction));
                    }
                }
            }
        }
    }

    // M.M | m1 = (0,0,down-right)
    // .A. | m2 = (0,2,down-left)
    // S.S | i1 == i1 && j1 + 2 = j2 && (d1 = down-right || d1 = down-left) && (d2 = down-left || d2 = down-right)

    // S.S | m1 = (2,0,up-right)
    // .A. | m2 = (2,2,up-left)
    // M.M | i1 == i2 && j1 + 2 == j2 && (d1 = up-right || d1 = up-left) && (d2 = up-left || d2 = up-right)

    // S.M | m1 = (0,2,down-left)
    // .A. | m2 = (2,2,up-left)
    // S.M | i1 + 2 == i2 && j1 == j2 && (d1 = down-left || d1 = up-left) && (d2 = up-left || d2 = down-left)

    // M.S | m1 = (0,0,down-right)
    // .A. | m2 = (2,0,up-right)
    // M.S | i1 + 2 == i2 && j1 == j2 && (d1 = down-right || d1 == up-right) && (d2 = down-right || d2 == up-right)

    let mut count = 0;

    for &o1 in occurrences.iter() {
        for &o2 in occurrences.iter() {
            if o1 != o2 {
                let (i1, j1, d1) = o1;
                let (i2, j2, d2) = o2;

                if d1 != d2 {
                    let d = vec![(d1, d2), (d2, d1)];
                    let vertical = i1 == i2 && j1 + 2 == j2;
                    let horizontal = i1 + 2 == i2 && j1 == j2;

                    // v && c1 || v && c2 || h && c3 || h && c4
                    // vc1 + vc2 + hc3 + hc4
                    // v(c1 + c2) + h(c3 + c4)
                    // [v && (c1 || c2)] || [h && (c3 || c4)]

                    let up = vertical && d.contains(&(Dir::Dr, Dir::Dl));
                    let down = vertical && d.contains(&(Dir::Ul, Dir::Ul));
                    let right = horizontal && d.contains(&(Dir::Dl, Dir::Ul));
                    let left = horizontal && d.contains(&(Dir::Dr, Dir::Ur));

                    if !!(up || down || right || left) {
                        let o = [o1, o2]
                            .iter()
                            .map(|f| format!("({},{},{})", f.0, f.1, f.2))
                            .collect::<Vec<_>>();
                        println!("{} \t {}", o[0], o[1]);

                        count += 1;
                    }
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
    direction: Dir,
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
