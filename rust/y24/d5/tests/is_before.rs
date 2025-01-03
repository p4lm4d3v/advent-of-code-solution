extern crate d5;

use d5::is_before::is_before;

#[test]
fn is_before1() {
    let result = is_before(&vec![75, 47, 61, 53, 29], 75, 47);
    assert!(result);
}
#[test]
fn is_before2() {
    let result = is_before(&vec![75, 47, 61, 53, 29], 75, 61);
    assert!(result);
}
#[test]
fn is_before3() {
    let result = is_before(&vec![75, 47, 61, 53, 29], 75, 53);
    assert!(result);
}
#[test]
fn is_before4() {
    let result = is_before(&vec![75, 47, 61, 53, 29], 75, 29);
    assert!(result);
}
#[test]
fn is_before5() {
    let result = is_before(&vec![75, 47, 61, 53, 29], 47, 61);
    assert!(result);
}
#[test]
fn is_before6() {
    let result = is_before(&vec![75, 47, 61, 53, 29], 47, 53);
    assert!(result);
}
#[test]
fn is_before7() {
    let result = is_before(&vec![75, 47, 61, 53, 29], 47, 29);
    assert!(result);
}
#[test]
fn is_before8() {
    let result = is_before(&vec![75, 47, 61, 53, 29], 61, 53);
    assert!(result);
}
#[test]
fn is_before9() {
    let result = is_before(&vec![75, 47, 61, 53, 29], 61, 29);
    assert!(result);
}
#[test]
fn is_before10() {
    let result = is_before(&vec![75, 47, 61, 53, 29], 53, 29);
    assert!(result);
}

#[test]
fn is_before11() {
    let result = is_before(&vec![97, 61, 53, 29, 13], 97, 61);
    assert!(result);
}
#[test]
fn is_before12() {
    let result = is_before(&vec![97, 61, 53, 29, 13], 97, 53);
    assert!(result);
}
#[test]
fn is_before13() {
    let result = is_before(&vec![97, 61, 53, 29, 13], 97, 29);
    assert!(result);
}
#[test]
fn is_before14() {
    let result = is_before(&vec![97, 61, 53, 29, 13], 97, 13);
    assert!(result);
}
#[test]
fn is_before15() {
    let result = is_before(&vec![97, 61, 53, 29, 13], 61, 53);
    assert!(result);
}
#[test]
fn is_before16() {
    let result = is_before(&vec![97, 61, 53, 29, 13], 61, 29);
    assert!(result);
}
#[test]
fn is_before17() {
    let result = is_before(&vec![97, 61, 53, 29, 13], 61, 13);
    assert!(result);
}
#[test]
fn is_before18() {
    let result = is_before(&vec![97, 61, 53, 29, 13], 53, 29);
    assert!(result);
}
#[test]
fn is_before19() {
    let result = is_before(&vec![97, 61, 53, 29, 13], 53, 13);
    assert!(result);
}

#[test]
fn is_before20() {
    let result = is_before(&vec![75, 29, 13], 75, 29);
    assert!(result);
}
#[test]
fn is_before21() {
    let result = is_before(&vec![75, 29, 13], 75, 13);
    assert!(result);
}
#[test]
fn is_before22() {
    let result = is_before(&vec![75, 29, 13], 29, 13);
    assert!(result);
}
