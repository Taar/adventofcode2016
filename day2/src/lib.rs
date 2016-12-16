pub mod part1 {
    pub fn bathroom_code (directions: &String) -> String {
        let mut movements  = directions.chars();
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
}

pub mod part2 {

    struct KeyPad {
        current: u8,
    }

    impl KeyPad {
        /*
            1
          2 3 4
        5 6 7 8 9
          A B C
            D 
        */
        fn move_up (&mut self) {
            match self.current {
                // 6, 7, 8
                54 | 55 | 56 => self.current -= 4,
                // A, B, C
                65 | 66 | 67 => self.current -= 11,
                // D, 3
                68 | 51 => self.current -= 2,
                // everything else
                _ => {}
            }
        }

        fn move_right (&mut self) {
            match self.current {
                // 1, 4, 9, C, D
                49 | 52 | 57 | 67 | 68 => {},
                // everything else
                _ => self.current += 1,
            }
        }

        fn move_left (&mut self) {
            match self.current {
                // 1, 2, 5, A, D
                49 | 50 | 53 | 65 | 68 => {},
                // everything else
                _ => self.current -= 1,
            }
        }

        fn move_down (&mut self) {
            match self.current {
                // 2, 3, 4
                50 | 51 | 52 => self.current += 4,
                // 6, 7, 8
                54 | 55 | 56 => self.current += 11,
                // B, 1
                66 | 49 => self.current += 2,
                // everything else
                _ => {}
            }
        }
    }

    pub fn bathroom_code (directions: &String) -> String {
        let mut movements = directions.chars();
        let mut character = movements.next();
        let mut code: Vec<u8> = Vec::new();
        // decimal of 5: 00053 or \u0035
        // We'll use this to convert it to the character later on
        let mut key_pad = KeyPad { current: 53 };

        while character != None {
            let c: char = character.unwrap();

            if c == '\n' {
                code.push(key_pad.current);
                character = movements.next();
                continue;
            }

            match c {
                'U' => key_pad.move_up(),
                'D' => key_pad.move_down(),
                'L' => key_pad.move_left(),
                'R' => key_pad.move_right(),
                _ => {},
            }

            character = movements.next();
        }

        return String::from_utf8(code).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::{ part1, part2 };

    #[test]
    fn code_1985() {
        let input = String::from("ULL\nRRDDD\nLURDL\nUUUUD\n");
        assert_eq!(String::from("1985"), part1::bathroom_code(&input));
    }

    #[test]
    fn code_4985() {
        let input = String::from("ULLLLD\nRRDDD\nLURDL\nUUUUD\n");
        assert_eq!(String::from("4985"), part1::bathroom_code(&input));
    }

    #[test]
    fn code_62936() {
        let mut input = String::new();
        input.push_str("LDUDDRUDRRU\n");
        input.push_str("DRURURLLUUR\n");
        input.push_str("URUUDUDRDDR\n");
        input.push_str("LRRLLRURUUR\n");
        input.push_str("DDLRRULRDUR\n");
        assert_eq!(String::from("62936"), part1::bathroom_code(&input));
    }

    #[test]
    fn code_5db3() {
        let input = String::from("ULL\nRRDDD\nLURDL\nUUUUD\n");
        assert_eq!(String::from("5DB3"), part2::bathroom_code(&input));
    }
}
