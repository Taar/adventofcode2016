extern crate regex;

use regex::Regex;
use std::io::{ BufReader, BufRead };


pub fn find_possible_triangles (input: String) -> u32 {
    let mut buffer = BufReader::new(input.as_bytes());
    let mut line = String::new();
    let mut count = 0;

    let numbers_re = Regex::new(r"(\d+)").unwrap();
    while buffer.read_line(&mut line).unwrap() > 0 {
        let mut sides: Vec<u32> = Vec::new(); 

        for capture in numbers_re.captures_iter(line.as_str()) {
            let number = capture.at(1).unwrap_or("");
            sides.push(number.parse::<u32>().unwrap());
        }

        if sides.len() < 3 {
            continue;
        }

        let &max = sides.iter().max().unwrap();
        let &min = sides.iter().min().unwrap();
        let sum_of_reminder: u32 = sides.iter().sum();

        let a = sum_of_reminder - max;
        let b = sum_of_reminder - min;
        let c = sum_of_reminder - min - max;

        if a + b > c && b + c > a && c + a > b {
            count += 1;
            println!("Sides: {:?}", sides);
        }

        line.clear();
    }

    return count;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn five_are_possible() {
        let mut input = String::new();
        input.push_str("    5   10   10 \n"); // true
        input.push_str("   10   10   10 \n"); // false
        input.push_str("   14   10   10 \n"); // true
        input.push_str("   45   10  120 \n"); // false
        input.push_str("  545  410  920 \n"); // true
        input.push_str("  345  410  520 \n"); // true
        input.push_str("   45  450  500 \n"); // false
        assert_eq!(5, find_possible_triangles(input));
    }

    #[test]
    fn zero_are_possible() {
        let mut input = String::new();
        input.push_str("  725  312  215 \n"); // false
        assert_eq!(0, find_possible_triangles(input));
    }
}
