pub fn bathroom_code (directions: String) -> String {
    let mut movements = directions.chars();
    let mut code: Vec<u8> = Vec::new();
    // decimal of 5: 00053 or \u0035
    // We'll use this to convert it to the character later on
    let mut button_number: u8 = 53;
    let mut character = movements.next();

    while character != None {
        let c: char = character.unwrap();

        if c == '\n' {
            code.push(button_number);
            character = movements.next();
            continue;
        }

        let mut result: u8 = button_number.clone();
        match c {
            'U' => result -= 3,
            'L' => {
                match result {
                    // 1, 4, 7
                    49 | 52 | 55 => {},
                    _ => result -= 1,
                }
            },
            'R' => {
                match result {
                    // 3, 6, 9
                    51 | 54 | 57 => {},
                    _ => result += 1,
                }
            },
            'D' => result += 3,
            _ => {},
        }

        // 49 = '1' and 57 = '9'
        if result >= 49 && result <= 57 {
            button_number = result.clone();
        }

        character = movements.next();
    }

    return String::from_utf8(code).unwrap();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn code_1985() {
        let input = String::from("ULL\nRRDDD\nLURDL\nUUUUD\n");
        assert_eq!(String::from("1985"), bathroom_code(input));
    }

    #[test]
    fn code_4985() {
        let input = String::from("ULLLLD\nRRDDD\nLURDL\nUUUUD\n");
        assert_eq!(String::from("4985"), bathroom_code(input));
    }

    #[test]
    fn code_62936() {
        let mut input = String::new();
        input.push_str("LDUDDRUDRRU\n");
        input.push_str("DRURURLLUUR\n");
        input.push_str("URUUDUDRDDR\n");
        input.push_str("LRRLLRURUUR\n");
        input.push_str("DDLRRULRDUR\n");
        assert_eq!(String::from("62936"), bathroom_code(input));
    }
}
