#[cfg(test)]
mod t1 {
    use crate::letter_twice_in_a_row;

    #[test]
    fn letter_twice_in_a_row_test1() {
        let result1 = letter_twice_in_a_row("aaa");
        assert_eq!(result1, true);
    }
    #[test]
    fn letter_twice_in_a_row_test2() {
        let result2 = letter_twice_in_a_row("aabbccdd");
        assert_eq!(result2, true);
    }

    #[test]
    fn letter_twice_in_a_row_test3() {
        let result3 = letter_twice_in_a_row("abc");

        assert_eq!(result3, false);
    }
}
