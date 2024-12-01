#[cfg(test)]
mod t2 {
    use crate::process;

    #[test]
    fn test1() {
        let result = process("3   4\n4   3\n2   5\n1   3\n3   9\n3   3");
        assert_eq!(result, 31);
    }

    #[test]
    fn test2() {
        let result = process("1   2\n2   3\n3   4\n4   5\n5   6\n6   7\n7   8\n8   9");
        assert_eq!(result, 35);
    }
}
