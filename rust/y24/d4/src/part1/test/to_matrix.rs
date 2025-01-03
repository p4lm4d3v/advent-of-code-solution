#[cfg(test)]
mod t1 {
    use crate::to_matrix;

    #[test]
    fn to_matrix1() {
        let result = to_matrix("");
        assert_eq!(result, vec![vec![]]);
    }

    #[test]
    fn to_matrix2() {
        let result = to_matrix("abc");
        assert_eq!(result, vec![vec!['a', 'b', 'c']]);
    }

    #[test]
    fn to_matrix3() {
        let result = to_matrix("abc\ndef");
        assert_eq!(result, vec![vec!['a', 'b', 'c'], vec!['d', 'e', 'f'],]);
    }

    #[test]
    fn to_matrix4() {
        let result = to_matrix("a b c\nd e f");
        assert_eq!(
            result,
            vec![vec!['a', '.', 'b', '.', 'c'], vec!['d', '.', 'e', '.', 'f'],]
        );
    }

    #[test]
    fn to_matrix5() {
        let result = to_matrix("a\nb\nc");
        assert_eq!(result, vec![vec!['a'], vec!['b'], vec!['c'],]);
    }

    #[test]
    fn to_matrix6() {
        let result = to_matrix("\n\n\n");
        assert_eq!(result, vec![vec![], vec![], vec![]]);
    }

    #[test]
    fn to_matrix7() {
        let result = to_matrix("123\n456\n789");
        assert_eq!(
            result,
            vec![
                vec!['1', '2', '3'],
                vec!['4', '5', '6'],
                vec!['7', '8', '9'],
            ]
        );
    }

    #[test]
    fn to_matrix8() {
        let result = to_matrix("a\nb\nc");
        assert_eq!(result, vec![vec!['a'], vec!['b'], vec!['c'],]);
    }

    #[test]
    fn to_matrix9() {
        let result = to_matrix("!@#\n$%^");
        assert_eq!(result, vec![vec!['!', '@', '#'], vec!['$', '%', '^'],]);
    }

    #[test]
    fn to_matrix10() {
        let result = to_matrix("a b \nc d ");
        assert_eq!(
            result,
            vec![vec!['a', '.', 'b', '.'], vec!['c', '.', 'd', '.'],]
        );
    }
}
