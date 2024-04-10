fn main() {
    let input: &str = include_str!("./input1.txt");
    let output = process(input);
    dbg!(output);
}

fn separate_digit_and_other(chars: &mut Vec<char>) -> &Vec<char> {
    for i in 0..chars.len() - 1 {
        let first_char = chars[i];
        let second_char = chars[i + 1];

        if first_char.is_digit(10) && !second_char.is_digit(10) && !second_char.is_whitespace()
        {
            chars.insert(i + 1, ' ');
        } else if second_char.is_digit(10)
            && !first_char.is_digit(10)
            && !first_char.is_whitespace()
        {
            chars.insert(i + 1, ' ');
        }
    }
    chars
}

fn process(input: &str) -> u32 {
    let lines: Vec<&str> = input.lines().collect();

    for l in lines {
        let line = l.replace(".", " ");
        let reduced_whitespace: String = line.split_whitespace().collect::<Vec<&str>>().join(" ");

        let mut chars: Vec<char> = reduced_whitespace.chars().collect();

        chars = separate_digit_and_other(&mut chars).clone();

        let str: String = chars.iter().collect();
        let mut split: Vec<&str> = str.split(" ").collect();

        let mut last_chars: Vec<char> = split.last().unwrap().chars().collect();
        split.remove(split.len() - 1);

        last_chars = separate_digit_and_other(&mut last_chars).clone();
        let q: String = last_chars.iter().collect();
        let mut split2: Vec<&str> = q.split(" ").collect();

        split.append(&mut split2);
        
        let mut remove_indices: Vec<usize> = Vec::new();
        for (i, c) in split.iter().enumerate() {
            if c.len() == 1 {
                remove_indices.push(i);                
            } 
        }

        println!();
        for ind in remove_indices {
            print!("{} ", split.get(ind).unwrap());
            split.remove(ind);
        }
        println!();

        print!("[");
        for (i, x) in split.iter().enumerate() {
            print!("{}{}", x, if i != split.len() - 1 { "," } else { "" })
        }
        print!("]");


        println!();
        // let new_string: String = chars.iter().collect();
        // println!("{}", new_string);
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = process(
            "467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..",
        );
        assert_eq!(result, 4361);
    }
}
