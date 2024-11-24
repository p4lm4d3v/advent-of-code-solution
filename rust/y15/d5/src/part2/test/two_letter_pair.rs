#[cfg(test)]
mod t2 {
    use crate::two_letter_pair;

    #[test]
    fn two_letter_pair_test1() {
        let result = two_letter_pair("qjhvhtzxzqqjkmpb");
        assert_eq!(result.0, true);
    }
    #[test]
    fn two_letter_pair_test2() {
        let result = two_letter_pair("xxyxx");
        assert_eq!(result.0, true);
    }
    #[test]
    fn two_letter_pair_test3() {
        let result = two_letter_pair("uurcxstgmygtbstg");
        assert_eq!(result.0, true);
    }
    #[test]
    fn two_letter_pair_test4() {
        let result = two_letter_pair("ieodomkazucvgmuy");
        assert_eq!(result.0, false);
    }
    #[test]
    fn two_letter_pair_test5() {
        let result = two_letter_pair("aaa");
        assert_eq!(result.0, false);
    }
}
