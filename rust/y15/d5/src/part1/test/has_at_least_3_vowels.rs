#[cfg(test)]
mod t1 {
    use crate::has_at_least_3_vowels;

    #[test]
    fn has_at_least_3_vowels_test1() {
        let result1 = has_at_least_3_vowels("aio");
        assert_eq!(result1, true);
    }
    #[test]
    fn has_at_least_3_vowels_test2() {
        let result2 = has_at_least_3_vowels("xazegov");
        assert_eq!(result2, true);
    }
    #[test]
    fn has_at_least_3_vowels_test3() {
        let result3 = has_at_least_3_vowels("pqr");
        assert_eq!(result3, false);
    }
    #[test]
    fn has_at_least_3_vowels_test4() {
        let result4 = has_at_least_3_vowels("aaa");
        assert_eq!(result4, true);
    }
}
