#[cfg(test)]
mod t1 {
    use crate::process;

    #[test]
    fn test1() {
        let result =
            process("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))");
        assert_eq!(result, 161);
    }
}
