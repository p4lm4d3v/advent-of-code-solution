#[cfg(test)]
mod t2 {
    use crate::remove_single_element;

    #[test]
    fn remove_single_element1() {
        let result = remove_single_element("7 6 4 2 1");
        assert_eq!(
            result,
            vec!["6 4 2 1", "7 4 2 1", "7 6 2 1", "7 6 4 1", "7 6 4 2"]
        );
    }

    #[test]
    fn remove_single_element2() {
        let result = remove_single_element("1 2 7 8 9");
        assert_eq!(
            result,
            vec!["2 7 8 9", "1 7 8 9", "1 2 8 9", "1 2 7 9", "1 2 7 8"]
        );
    }

    #[test]
    fn remove_single_element3() {
        let result = remove_single_element("9 7 6 2 1");
        assert_eq!(
            result,
            vec!["7 6 2 1", "9 6 2 1", "9 7 2 1", "9 7 6 1", "9 7 6 2"]
        );
    }

    #[test]
    fn remove_single_element4() {
        let result = remove_single_element("1 3 2 4 5");
        assert_eq!(
            result,
            vec!["3 2 4 5", "1 2 4 5", "1 3 4 5", "1 3 2 5", "1 3 2 4"]
        );
    }

    #[test]
    fn remove_single_element5() {
        let result = remove_single_element("8 6 4 4 1");
        assert_eq!(
            result,
            vec!["6 4 4 1", "8 4 4 1", "8 6 4 1", "8 6 4 1", "8 6 4 4"]
        );
    }

    #[test]
    fn remove_single_element6() {
        let result = remove_single_element("1 3 6 7 9");
        assert_eq!(
            result,
            vec!["3 6 7 9", "1 6 7 9", "1 3 7 9", "1 3 6 9", "1 3 6 7"]
        );
    }
}
