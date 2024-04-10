use std::fs::File;
use std::io::{self, Read, Write};

static UP_LEFT: char = '┌';
static UP_RIGHT: char = '┐';
static DOWN_LEFT: char = '└';
static DOWN_RIGHT: char = '┘';
static HORIZONTAL: char = '─';
static VERTICAL: char = '│';

fn main() -> io::Result<()> {
    // Open the input file
    let input_file_path = "src\\bin\\input1.txt"; // Change this to your input file path
    let mut input_file = File::open(input_file_path)?;

    // Read the content of the input file
    let mut content = String::new();
    input_file.read_to_string(&mut content)?;

    // Perform text replacements
    let modified_content = content
        .replace("F", UP_LEFT.to_string().as_str())
        .replace("7", UP_RIGHT.to_string().as_str())
        .replace("L", DOWN_LEFT.to_string().as_str())
        .replace("J", DOWN_RIGHT.to_string().as_str())
        .replace("-", HORIZONTAL.to_string().as_str())
        .replace("|", VERTICAL.to_string().as_str());

    // Create the output file
    let output_file_path = "src\\bin\\output.txt"; // Change this to your desired output file path
    let mut output_file = File::create(output_file_path)?;

    // Write the modified content to the output file
    output_file.write_all(modified_content.as_bytes())?;

    println!("File successfully processed. Modified content written to output.txt.");

    Ok(())
}
