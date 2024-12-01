#[cfg(test)]
mod t2 {
    use crate::count_in;

    #[test]
    fn count_in1() {
        let result = count_in(1, &vec![1, 1, 1, 1, 1, 1, 1, 1]);
        assert_eq!(result, 8);
    }
    #[test]
    fn count_in2() {
        let result = count_in(
            8,
            &vec![1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9],
        );
        assert_eq!(result, 2);
    }
    #[test]
    fn count_in3() {
        let result = count_in(3, &vec![4, 3, 5, 3, 9, 3]);
        assert_eq!(result, 3);
    }
}
