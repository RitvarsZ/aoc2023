use std::fs;

fn main() {
    let path = "src/input";
    let contents = fs::read_to_string(path)
        .expect("Should have been able to read the file.");

    let mut sum = 0;
    for line in contents.lines() {
        // Find first and last digit
        let mut first: char = '0';
        let mut last: char = '0';
        for (_i, c) in line.chars().enumerate() {
            if c.is_digit(10) {
                if first == '0' {
                    first = c;
                }
                last = c;
            }
        }

        let num:i32 = format!("{first}{last}").parse()
            .expect("Should have been able to parse num.");

        println!("{num}");
        sum += num
    }

    println!("{sum}");
}
