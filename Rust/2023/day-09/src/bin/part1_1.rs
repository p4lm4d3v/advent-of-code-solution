fn main() {
    let input: &str = include_str!("./input1.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> i128 {
    let lines = input.lines().map(|l| l.trim()).collect::<Vec<_>>();

    let mut sums: Vec<i128> = Vec::new();
    for line in lines.iter() {
        let values = line
            .split(" ")
            .map(|c| c.parse().unwrap())
            .collect::<Vec<i128>>();

        let mut dataset: Vec<Vec<i128>> = vec![values.clone()];

        while !dataset.last().unwrap().iter().all(|n| *n == 0) {
            let last = dataset.last().unwrap();
            let new_values = last
                .windows(2)
                .map(|pair| pair[1] - pair[0])
                .collect::<Vec<i128>>();
            dataset.push(new_values);
        }

        let sum = dataset
            .iter()
            .map(|d1| *d1.last().ok_or(format!("Length: {}", d1.len())).unwrap())
            .collect::<Vec<_>>()
            .iter()
            .fold(0, |u1, u2| u1 + u2);

        sums.push(sum);
        // printds(dataset);
        // println!("{}", sum);
    }

    sums.iter().fold(0, |s1, s2| s1 + s2)
}

fn printds(ds: Vec<Vec<i128>>) {
    let temp = ds
        .iter()
        .map(|d1| {
            d1.iter()
                .map(|d2| d2.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        })
        .collect::<Vec<String>>()
        .join("\n");

    println!("{}", temp);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_1_test1() {
        let result = process(
            "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45",
        );
        assert_eq!(result, 114);
    }
}
