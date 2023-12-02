use std::{fs,collections::HashMap};

fn main() {
    let path = "src/input";
    let contents = fs::read_to_string(path)
        .expect("Should have been able to read the file.");

    let mut digits = HashMap::new();
    digits.insert("one", 1);
    digits.insert("two", 2);
    digits.insert("three", 3);
    digits.insert("four", 4);
    digits.insert("five", 5);
    digits.insert("six", 6);
    digits.insert("seven", 7);
    digits.insert("eight", 8);
    digits.insert("nine", 9);
    digits.insert("zero", 0);
    digits.insert("1", 1);
    digits.insert("2", 2);
    digits.insert("3", 3);
    digits.insert("4", 4);
    digits.insert("5", 5);
    digits.insert("6", 6);
    digits.insert("7", 7);
    digits.insert("8", 8);
    digits.insert("9", 9);
    digits.insert("0", 0);

    let mut sum = 0;
    for line in contents.lines() {
        let mut first: Option<(i32, usize)> = None;
        let mut last: Option<(i32, usize)> = None;

        for (digit, n) in digits.iter() {
            if line.contains(digit) {
                let first_idx = line.find(digit).unwrap();
                if first == None || first.unwrap().1 > first_idx {
                    first = Some((*n, first_idx));
                }

                let last_idx = line.rfind(digit).unwrap();
                if last == None || last.unwrap().1 < last_idx {
                    last = Some((*n, last_idx));
                }
            }
        }

        let num:i32 = format!(
            "{}{}",
            first.unwrap_or((0,0)).0,
            last.unwrap_or((0,0)).0
        ).parse().expect("Should have been able to parse num.");

        println!("{num}");
        sum += num
    }

    println!("{sum}");
}
