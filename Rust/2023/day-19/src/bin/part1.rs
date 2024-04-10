use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    str::MatchIndices,
    thread::sleep,
    time::Duration,
};

use iter_tools::Itertools;

fn main() {
    let input: &str = include_str!("./input1.txt");
    let output = process(input);
    dbg!(output);
}

fn parse_workflows(input: &str) -> HashMap<String, Workflow> {
    let mut workflows: Vec<Workflow> = Vec::new();

    let str = input.split("\n\n").collect::<Vec<_>>();
    let parts_str = str[0..1].to_vec().iter().map(|f| *f).collect::<String>();
    let parts_strs = parts_str.split("\n").collect::<Vec<_>>();
    for ps in parts_strs.iter() {
        let mut rules: Vec<Rule> = Vec::new();

        let (name, mut right) = ps
            .split("{")
            .map(|f| f.to_string())
            .collect_tuple()
            .unwrap();
        right = right.replace("}", "");
        let rules_strs = right.split(",").collect::<Vec<_>>();
        for rs in rules_strs {
            if rs.contains("<") {
                let (attribute, right) = rs.split("<").collect_tuple().unwrap();
                let attr = match attribute {
                    "x" => Attribute::X,
                    "m" => Attribute::M,
                    "a" => Attribute::A,
                    "s" => Attribute::S,
                    _ => panic!("Unknown attribute: {}", attribute),
                };
                let (n, t) = right.split(":").collect_tuple().unwrap();
                let num = n.parse::<u32>().unwrap();
                let to = match t {
                    "A" => To::Accept,
                    "R" => To::Reject,
                    _ => To::Next(t.to_string()),
                };
                rules.push(Rule::Less(attr, num, to));
            } else if rs.contains(">") {
                let (attribute, right) = rs.split(">").collect_tuple().unwrap();
                let attr = match attribute {
                    "x" => Attribute::X,
                    "m" => Attribute::M,
                    "a" => Attribute::A,
                    "s" => Attribute::S,
                    _ => panic!("Unknown attribute: {}", attribute),
                };
                let (n, t) = right.split(":").collect_tuple().unwrap();
                let num = n.parse::<u32>().unwrap();
                let to = match t {
                    "A" => To::Accept,
                    "R" => To::Reject,
                    _ => To::Next(t.to_string()),
                };
                rules.push(Rule::Greater(attr, num, to));
            } else {
                let next = match rs {
                    "A" => To::Accept,
                    "R" => To::Reject,
                    _ => To::Next(rs.to_string()),
                };
                rules.push(Rule::Next(next));
            }
        }
        workflows.push(Workflow { name, rules });
    }
    let mut map: HashMap<String, Workflow> = HashMap::new();
    for w in workflows {
        map.insert(w.name.clone(), w);
    }
    map
}
fn parse_parts(input: &str) -> Vec<Part> {
    let mut parts: Vec<Part> = Vec::new();

    let str = input.split("\n\n").collect::<Vec<_>>();
    let parts_str = str[1..].to_vec().iter().map(|f| *f).collect::<String>();
    let parts_strs = parts_str.split("\n").collect::<Vec<_>>();

    for ps in parts_strs.iter() {
        let mut s = ps.to_string();
        s = s
            .replace("{", "")
            .replace("}", "")
            .replace("x=", "")
            .replace("m=", "")
            .replace("a=", "")
            .replace("s=", "");
        let temp = s.split(",").collect::<Vec<_>>();
        if temp.len() == 4 {
            let (x, m, a, s) = temp
                .iter()
                .map(|f| f.parse::<u32>().unwrap())
                .collect_tuple()
                .unwrap();
            parts.push(Part { x, m, a, s });
        }
    }
    parts
}
fn get_accepted(workflows: HashMap<String, Workflow>, parts: Vec<Part>) -> Vec<Part> {
    let mut accepted: Vec<Part> = Vec::new();
    let mut key = "in".to_string();
    for p in parts.iter() {
        'lup: loop {
            if let Some(w) = workflows.get(&key) {
                for r in w.rules.iter() {
                    match r {
                        Rule::Greater(att, val, to) => match att {
                            Attribute::X => {
                                if p.x > *val {
                                    match to {
                                        To::Next(n) => key = n.clone(),
                                        To::Accept => {
                                            let new = Part {
                                                x: p.x,
                                                m: p.m,
                                                a: p.a,
                                                s: p.s,
                                            };
                                            accepted.push(new);
                                            break 'lup;
                                        }
                                        To::Reject => break 'lup,
                                    }
                                }
                            }
                            Attribute::M => {
                                if p.m > *val {
                                    match to {
                                        To::Next(n) => key = n.clone(),
                                        To::Accept => {
                                            let new = Part {
                                                x: p.x,
                                                m: p.m,
                                                a: p.a,
                                                s: p.s,
                                            };
                                            accepted.push(new);
                                            break 'lup;
                                        }
                                        To::Reject => break 'lup,
                                    }
                                }
                            }
                            Attribute::A => {
                                if p.a > *val {
                                    match to {
                                        To::Next(n) => key = n.clone(),
                                        To::Accept => {
                                            let new = Part {
                                                x: p.x,
                                                m: p.m,
                                                a: p.a,
                                                s: p.s,
                                            };
                                            accepted.push(new);
                                            break 'lup;
                                        }
                                        To::Reject => break 'lup,
                                    }
                                }
                            }
                            Attribute::S => {
                                if p.s > *val {
                                    match to {
                                        To::Next(n) => key = n.clone(),
                                        To::Accept => {
                                            let new = Part {
                                                x: p.x,
                                                m: p.m,
                                                a: p.a,
                                                s: p.s,
                                            };
                                            accepted.push(new);
                                            break 'lup;
                                        }
                                        To::Reject => break 'lup,
                                    }
                                }
                            }
                        },
                        Rule::Less(att, val, to) => match att {
                            Attribute::X => {
                                if p.x < *val {
                                    match to {
                                        To::Next(n) => key = n.clone(),
                                        To::Accept => {
                                            let new = Part {
                                                x: p.x,
                                                m: p.m,
                                                a: p.a,
                                                s: p.s,
                                            };
                                            accepted.push(new);
                                            break 'lup;
                                        }
                                        To::Reject => break 'lup,
                                    }
                                }
                            }
                            Attribute::M => {
                                if p.m < *val {
                                    match to {
                                        To::Next(n) => key = n.clone(),
                                        To::Accept => {
                                            let new = Part {
                                                x: p.x,
                                                m: p.m,
                                                a: p.a,
                                                s: p.s,
                                            };
                                            accepted.push(new);
                                            break 'lup;
                                        }
                                        To::Reject => break 'lup,
                                    }
                                }
                            }
                            Attribute::A => {
                                if p.a < *val {
                                    match to {
                                        To::Next(n) => key = n.clone(),
                                        To::Accept => {
                                            let new = Part {
                                                x: p.x,
                                                m: p.m,
                                                a: p.a,
                                                s: p.s,
                                            };
                                            accepted.push(new);
                                            break 'lup;
                                        }
                                        To::Reject => break 'lup,
                                    }
                                }
                            }
                            Attribute::S => {
                                if p.s < *val {
                                    match to {
                                        To::Next(n) => key = n.clone(),
                                        To::Accept => {
                                            let new = Part {
                                                x: p.x,
                                                m: p.m,
                                                a: p.a,
                                                s: p.s,
                                            };
                                            accepted.push(new);
                                            break 'lup;
                                        }
                                        To::Reject => break 'lup,
                                    }
                                }
                            }
                        },
                        Rule::Next(to) => match to {
                            To::Next(n) => key = n.clone(),
                            To::Accept => {
                                let new = Part {
                                    x: p.x,
                                    m: p.m,
                                    a: p.a,
                                    s: p.s,
                                };
                                accepted.push(new);
                                break 'lup;
                            }
                            To::Reject => break 'lup,
                        },
                    }
                }
            }
        }
    }
    accepted
}
fn process(input: &str) -> usize {
    let parts: Vec<Part> = parse_parts(input);
    let workflows: HashMap<String, Workflow> = parse_workflows(input);
    let accepted = get_accepted(workflows, parts);
    _print_parts(&accepted);
    0
}

fn _print_parts(parts: &Vec<Part>) {
    for p in parts.iter() {
        println!("{:?}", p);
    }
}
fn _print_workflows(workflows: &HashMap<String, Workflow>) {
    for (n, w) in workflows {
        println!("{}", n);
        for r in w.rules.iter() {
            println!("{:?}", r);
        }
        println!("");
    }
}

#[derive(Debug)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

#[derive(Debug)]
enum Rule {
    Greater(Attribute, u32, To),
    Less(Attribute, u32, To),
    Next(To),
}

#[derive(Debug)]
enum Attribute {
    X,
    M,
    A,
    S,
}

#[derive(Debug)]
enum To {
    Next(String),
    Accept,
    Reject,
}

#[derive(Debug)]
struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

impl Part {
    fn sum(self) -> u32 {
        self.x + self.m + self.a + self.s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_parts_test1() {
        let result = parse_parts(
            "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}",
        );
        assert_eq!(result.len(), 5);
    }

    #[test]
    fn parse_workflows_test1() {
        let result = parse_workflows(
            "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}",
        );
        assert_eq!(result.len(), 11);
    }

    #[test]
    fn process_test1() {
        let result = process(
            "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}",
        );
        assert_eq!(result, 19114)
    }
}
