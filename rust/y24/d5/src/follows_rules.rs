use is_before::is_before;
use typedef::{Rules, Update};

pub fn follows_rules(update: &Update, rules: &Rules) -> bool {
    rules
        .iter()
        .map(|&(x, y)| {
            if !update.contains(&x) || !update.contains(&y) {
                return true;
            }

            is_before(update, x, y)
        })
        .all(|b| b == true)
}
