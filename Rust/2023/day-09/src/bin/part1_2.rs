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

        let mut new_dataset = dataset.clone();

        let mut i = new_dataset.len() - 1;

        new_dataset[i].push(0);
        loop {
            if i == 0 {
                break;
            }

            let nds = new_dataset.clone();
            let u = nds[i].last().unwrap();
            let t = nds[i - 1].last().unwrap();

            new_dataset[i - 1].push(u + t);

            i -= 1;
        }

        printds(&new_dataset);

        let sum = *new_dataset[0].last().unwrap();
        sums.push(sum);
    }

    sums.iter().fold(0, |s1, s2| s1 + s2)
}

fn printds(ds: &Vec<Vec<i128>>) {
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

    println!("{}\n", temp);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_2_test1() {
        let result = process(
            "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45",
        );
        assert_eq!(result, 114);
    }
}
