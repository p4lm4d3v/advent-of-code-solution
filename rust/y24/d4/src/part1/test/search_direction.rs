#[cfg(test)]
mod t1 {
    use crate::{search_direction, to_matrix, Direction};

    #[test]
    fn search_direction1() {
        let result = search_direction(
            &to_matrix("..X...\n.SAMX.\n.A..A.\nXMAS.S\n.X...."),
            "XMAS",
            0,
            2,
            Direction::DownRight,
        );
        assert!(result);
    }

    #[test]
    fn search_direction2() {
        let result = search_direction(
            &to_matrix("..X...\n.SAMX.\n.A..A.\nXMAS.S\n.X...."),
            "XMAS",
            1,
            4,
            Direction::Left,
        );
        assert!(result);
    }

    #[test]
    fn search_direction3() {
        let result = search_direction(
            &to_matrix("..X...\n.SAMX.\n.A..A.\nXMAS.S\n.X...."),
            "XMAS",
            0,
            0,
            Direction::UpLeft,
        );
        assert!(!result);
    }

    #[test]
    fn search_direction4() {
        let result = search_direction(
            &to_matrix("..X...\n.SAMX.\n.A..A.\nXMAS.S\n.X...."),
            "XMAS",
            3,
            0,
            Direction::Right,
        );
        assert!(result);
    }

    #[test]
    fn search_direction5() {
        let result = search_direction(
            &to_matrix(
                "....XXMAS.\n.SAMXMS...\n...S..A...\n..A.A.MS.X\nXMASAMX.MM\nX.....XA.A\nS.S.S.S.SS\n.A.A.A.A.A\n..M.M.M.MM\n.X.X.XMASX"
            ), 
            "XMAS",
            0,  
            5, 
            Direction::Right,
        );
        assert!(result);
    }
}
