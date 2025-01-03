#[cfg(test)]
mod t2 {
    use crate::{search_direction, to_matrix, Dir};

    #[test]
    fn search_direction1() {
        let result = search_direction(&to_matrix("M.S\n.A.\nM.S"), "MAS", 0, 0, Dir::Dr);
        assert!(result);
    }

    #[test]
    fn search_direction2() {
        let result = search_direction(&to_matrix("M.S\n.A.\nM.S"), "MAS", 2, 0, Dir::Ur);
        assert!(result);
    }

    #[test]
    fn search_direction3() {
        let result = search_direction(&to_matrix("M.S\n.A.\nM.S"), "MAS", 0, 0, Dir::R);
        assert!(!result);
    }

    #[test]
    fn search_direction4() {
        let result = search_direction(
            &to_matrix("S.S.S.S.S.\n.A.A.A.A..\nM.M.M.M.M."),
            "MAS",
            2,
            0,
            Dir::Ur,
        );
        assert!(result);
    }

    #[test]
    fn search_direction5() {
        let result = search_direction(
            &to_matrix("S.S.S.S.S.\n.A.A.A.A..\nM.M.M.M.M."),
            "MAS",
            2,
            2,
            Dir::Ul,
        );
        assert!(result);
    }

    #[test]
    fn search_direction6() {
        let result = search_direction(
            &to_matrix("S.S.S.S.S.\n.A.A.A.A..\nM.M.M.M.M."),
            "MAS",
            2,
            2,
            Dir::Ur,
        );
        assert!(result);
    }

    #[test]
    fn search_direction7() {
        let result = search_direction(
            &to_matrix("S.S.S.S.S.\n.A.A.A.A..\nM.M.M.M.M."),
            "MAS",
            2,
            4,
            Dir::Ul,
        );
        assert!(result);
    }

    #[test]
    fn search_direction8() {
        let result = search_direction(
            &to_matrix("S.S.S.S.S.\n.A.A.A.A..\nM.M.M.M.M."),
            "MAS",
            2,
            4,
            Dir::Ur,
        );
        assert!(result);
    }

    #[test]
    fn search_direction9() {
        let result = search_direction(
            &to_matrix("S.S.S.S.S.\n.A.A.A.A..\nM.M.M.M.M."),
            "MAS",
            2,
            6,
            Dir::Ul,
        );
        assert!(result);
    }

    #[test]
    fn search_direction10() {
        let result = search_direction(
            &to_matrix("S.S.S.S.S.\n.A.A.A.A..\nM.M.M.M.M."),
            "MAS",
            2,
            6,
            Dir::Ur,
        );
        assert!(result);
    }

    #[test]
    fn search_direction11() {
        let result = search_direction(
            &to_matrix("S.S.S.S.S.\n.A.A.A.A..\nM.M.M.M.M."),
            "MAS",
            2,
            8,
            Dir::Ul,
        );
        assert!(result);
    }
}
