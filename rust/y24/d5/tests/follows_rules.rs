extern crate d5;

use d5::{follows_rules, get_rules};

use follows_rules::follows_rules;
use get_rules::get_rules;

#[test]
fn follows_rules1() {
    let update = vec![75, 47, 61, 53, 29];
    let result = follows_rules(&update, &get_rules());
    assert!(result);
}
#[test]
fn follows_rules2() {
    let update = vec![97, 61, 53, 29, 13];
    let result = follows_rules(&update, &get_rules());
    assert!(result);
}
#[test]
fn follows_rules3() {
    let update = vec![75, 29, 13];
    let result = follows_rules(&update, &get_rules());
    assert!(result);
}
#[test]
fn follows_rules4() {
    let update = vec![75, 97, 47, 61, 53];
    let result = follows_rules(&update, &get_rules());
    assert!(!result);
}
#[test]
fn follows_rules6() {
    let update = vec![97, 13, 75, 29, 47];
    let result = follows_rules(&update, &get_rules());
    assert!(!result);
}
#[test]
fn follows_rules5() {
    let update = vec![61, 13, 29];
    let result = follows_rules(&update, &get_rules());
    assert!(!result);
}
