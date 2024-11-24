mod test;

fn main() {
    let input = include_str!("./input2.txt");
    let output = process(input);
    dbg!("{}", output);
}

fn two_letter_pair(input: &str) -> (bool, String) {
    let chars = input.chars().enumerate().collect::<Vec<_>>();

    let pairs = chars
        .windows(2)
        .enumerate()
        // .map(|pair| pair[0].to_string() + pair[1].to_string().as_str())
        .collect::<Vec<_>>();

    let mut counter = 0;
    let mut out = "".to_string();
    for p1 in pairs.iter() {
        for p2 in pairs.iter() {
            let (_, pair_1st) = p1;
            let (_, pair_2nd) = p2;

            let (pair_1st_1st, pair_1st_2nd) = (pair_1st[0], pair_1st[1]);
            let (pair_2nd_1st, pair_2nd_2nd) = (pair_2nd[0], pair_2nd[1]);

            if pair_1st_1st != pair_1st_2nd {
                if pair_1st_2nd.0 != pair_2nd_1st.0
                    && pair_1st_1st != pair_2nd_1st
                    && pair_1st_2nd != pair_2nd_2nd
                    && pair_1st_1st != pair_2nd_2nd
                {
                    if pair_1st_1st.1 == pair_2nd_1st.1 && pair_1st_2nd.1 == pair_2nd_2nd.1 {
                        counter += 1;
                        let format = format!(
                            "{}, {}",
                            p1.1[0].1.to_string() + p2.1[0].1.to_string().as_str(),
                            p1.1[1].1.to_string() + p2.1[1].1.to_string().as_str()
                        );
                        out = format;
                    }
                }
            }
        }
    }

    (counter >= 1, out)
}

fn contains_palindrome(input: &str) -> (bool, String) {
    let chars = input.chars().collect::<Vec<_>>();

    let mut counter = 0;
    let mut out = "".to_string();
    for triplet in chars.windows(3) {
        if triplet[0] == triplet[2] {
            counter += 1;
            out = format!(
                "{}{}{}",
                triplet[0].clone(),
                triplet[1].clone(),
                triplet[2].clone()
            );
        };
    }

    (counter >= 1, out)
}

fn is_nice(input: &str) -> (bool, String, String) {
    let v1 = two_letter_pair(input);
    let v2 = contains_palindrome(input);

    (v1.0 && v2.0, v1.1, v2.1)
}

fn process(input: &str) -> usize {
    let lines = input.lines().map(|l| l.trim()).collect::<Vec<_>>();

    let mut nice_counter = 0;
    for line in lines {
        let is_nice = is_nice(line);
        if is_nice.0 {
            nice_counter += 1;
        }
    }

    nice_counter
}
