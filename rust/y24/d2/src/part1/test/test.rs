#[cfg(test)]
mod t1 {
    use crate::process;

    #[test]
    fn process1() {
        let result = process("7 6 4 2 1\n1 2 7 8 9\n9 7 6 2 1\n1 3 2 4 5\n8 6 4 4 1\n1 3 6 7 9");
        assert_eq!(result, 2);
    }

    #[test]
    fn process2() {
        let result = process("7 6 4 2 1\n1 2 3 4 5\n9 7 6 2 1\n1 3 2 4 5\n8 6 4 4 1\n1 3 6 7 9");
        assert_eq!(result, 3);
    }

    #[test]
    fn process3() {
        let result = process("7 6 4 2 1\n1 2 3 4 5\n9 7 6 2 1\n1 3 2 4 5\n8 6 4 2 1\n1 3 6 7 9");
        assert_eq!(result, 4);
    }
}
