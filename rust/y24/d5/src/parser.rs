use typedef::{Rules, Updates};

pub struct Parser {}

impl Parser {
    pub fn parse(input: &str) -> (Rules, Updates) {
        let (rules_str, updates_str) = Self::separate(input);

        let rules = Self::into_rules(rules_str);
        let updates = Self::into_updates(updates_str);

        (rules, updates)
    }

    fn separate(input: &str) -> (Vec<&str>, Vec<&str>) {
        let mut rules_str: Vec<&str> = vec![];
        let mut updates_str: Vec<&str> = vec![];

        let mut section: bool = false;
        for line in input.lines() {
            if line.is_empty() {
                section = true;
                continue;
            }

            let line = line.trim();
            match section {
                false => rules_str.push(line),
                true => updates_str.push(line),
            }
        }

        (rules_str, updates_str)
    }

    fn into_rules(rules_str: Vec<&str>) -> Rules {
        rules_str
            .iter()
            .map(|&rule_str| {
                let (x, y) = rule_str.split_once("|").unwrap();
                (
                    x.parse::<i32>()
                        .expect(format!("Couldn't parse: {}", x).as_str()),
                    y.parse::<i32>()
                        .expect(format!("Couldn't parse: {}", y).as_str()),
                )
            })
            .collect()
    }

    fn into_updates(updates_str: Vec<&str>) -> Updates {
        updates_str
            .iter()
            .map(|&update_str| {
                let nums_str: Vec<&str> = update_str.split(",").collect();
                nums_str
                    .iter()
                    .map(|num_str| num_str.parse().unwrap())
                    .collect()
            })
            .collect()
    }
}
