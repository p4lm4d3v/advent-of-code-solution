mod test1;

fn main() {
    let output = process("bgvyzdsv");
    dbg!("{}", output);
}

fn md5(input: &str) -> String {
    let digest = md5::compute(input);
    format!("{:x}", digest)
}

fn process(input: &str) -> usize {
    let mut number = 0;
    let mut hash = md5("");

    while !hash.starts_with("00000") {
        number += 1;

        let x = input.to_string() + number.to_string().as_str();
        hash = md5(&x);
    }

    number
}
