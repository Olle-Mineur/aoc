use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut sum = 0;

    for line in reader.lines() {
        let mut index_left = 0;
        let mut index_right = 0;

        for c in line?.chars() {
            if c.is_ascii_digit() {
                if index_left == 0 {
                    index_left = c as u32 - 48;
                    index_right = c as u32 - 48;
                } else {
                    index_right = c as u32 - 48;
                }
            }
        }
        println!("{}", index_left);
        println!("{}", index_right);
        sum += index_left*10 + index_right;
    }
    println!("{}", sum);

    Ok(())
}