#[cfg(test)]
mod t1 {
    use crate::does_not_contain;

    #[test]
    fn does_not_contain_test1() {
        let result1 = does_not_contain("ab");
        assert_eq!(result1, false);
    }
    #[test]
    fn does_not_contain_test2() {
        let result2 = does_not_contain("abcdpqxy");
        assert_eq!(result2, false);
    }
    #[test]
    fn does_not_contain_test3() {
        let result3 = does_not_contain("godhsr");
        assert_eq!(result3, true);
    }
}
