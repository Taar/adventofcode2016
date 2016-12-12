extern crate regex;

pub mod row {
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
}


pub mod column {
    use regex::Regex;
    use std::io::{ BufReader, BufRead };

    fn is_triangle (sides: Vec<u32>) -> bool {
        let &max = sides.iter().max().unwrap();
        let &min = sides.iter().min().unwrap();
        let sum_of_reminder: u32 = sides.iter().sum();

        let a = sum_of_reminder - max;
        let b = sum_of_reminder - min;
        let c = sum_of_reminder - min - max;

        return a + b > c && b + c > a && c + a > b;
    }

    pub fn find_possible_triangles (input: String) -> u32 {
        let mut buffer = BufReader::new(input.as_bytes());
        let mut line = String::new();
        let mut count = 0;
        let mut line_count = 0;

        let numbers_re = Regex::new(r"(\d+)").unwrap();
        while buffer.read_line(&mut line).unwrap() > 0 {

            if line_count < 2 {
                line_count += 1;
                continue;
            }

            line_count = 0;

            let mut sides: Vec<u32> = Vec::new(); 

            for capture in numbers_re.captures_iter(line.as_str()) {
                let number = capture.at(1).unwrap_or("");
                sides.push(number.parse::<u32>().unwrap());
            }

            if sides.len() < 9 {
                continue;
            }

            // Partition the sides by mod 3
            // Consider the following:
            // 0,1,2
            // 3,4,5
            // 6,7,8
            // We can partition the first "column" by adding 3 to each value
            // and checking if it can be divided by 3
            // We are are now left with a vector that looks like this:
            // 1,2
            // 4,5
            // 7,8
            // Again if we add 1 to each value and check if that value can be
            // divided by 3 we'll get the third column.
            let (first, remainder): (Vec<(usize, u32)>, Vec<(usize, u32)>) =
                sides.into_iter()
                .enumerate()
                // 0 is the index, 1 is the u32 value
                .partition(|&t| (t.0 + 3) % 3 == 0);

            let (third, second): (Vec<(usize, u32)>, Vec<(usize, u32)>) =
                remainder.into_iter()
                // 0 is the index, 1 is the u32 value
                .partition(|&t| (t.0 + 1) % 3 == 0);

            // Maybe could throw this into a loop but it's a fixed amount
            // and probably not worth the trouble
            if is_triangle(first.into_iter().map(|t| t.1).collect()) {
                count += 1
            }

            if is_triangle(second.into_iter().map(|t| t.1).collect()) {
                count += 1
            }

            if is_triangle(third.into_iter().map(|t| t.1).collect()) {
                count += 1
            }

            line.clear();
        }

        return count;
    }
}


#[cfg(test)]
mod tests {
    use super::{row, column};

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
        assert_eq!(5, row::find_possible_triangles(input));
    }

    #[test]
    fn zero_are_possible() {
        let mut input = String::new();
        input.push_str("  725  312  215 \n"); // false
        assert_eq!(0, row::find_possible_triangles(input));
    }

    #[test]
    fn c_five_are_possible() {
        let mut input = String::new();
        input.push_str("  101  201  301\n"); // true
        input.push_str("  102  202  302\n"); // true
        input.push_str("  103  203  303\n"); // true
        input.push_str("  401  501  201\n"); // true
        input.push_str("  402  502  602\n"); // true
        input.push_str("  403  503  103\n"); // true
        assert_eq!(5, column::find_possible_triangles(input));
    }
}
