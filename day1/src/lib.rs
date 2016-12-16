mod common {
    pub enum Cardinal {
        North,
        East,
        South,
        West,
    }

    pub struct Location {
        pub x: i32,
        pub y: i32,
        pub facing: Cardinal,
    }

    impl Location {
        pub fn get_position_as_string (&self) -> String {
            return format!("x{}y{}", self.x, self.y);
        }
    }
}

pub mod part1 {
    use common::{Cardinal, Location};

    pub fn blocks_away (directions: &String) -> i32 {
        let mut chars = directions.chars();
        let mut character = chars.next();
        let mut location = Location { x: 0, y: 0, facing: Cardinal::North};

        while character != None {
            let mut c: char = character.unwrap();
            if c == 'R' {
                match location.facing {
                    Cardinal::North => location.facing = Cardinal::East,
                    Cardinal::East => location.facing = Cardinal::South,
                    Cardinal::South => location.facing = Cardinal::West,
                    Cardinal::West => location.facing = Cardinal::North,
                }
            } else if c == 'L' {
                match location.facing {
                    Cardinal::North => location.facing = Cardinal::West,
                    Cardinal::East => location.facing = Cardinal::North,
                    Cardinal::South => location.facing = Cardinal::East,
                    Cardinal::West => location.facing = Cardinal::South,
                }
            } else {
                let mut number_of_blocks = String::new();
                loop {
                    match c.to_digit(10) {
                        Some(_) => number_of_blocks.push(c),
                        None => break,
                    }
                    character = chars.next();
                    match character {
                        Some(new_c) => c = new_c,
                        None => break,
                    }
                }
                if number_of_blocks.len() > 0 {
                    let blocks_to_move = number_of_blocks.parse::<i32>().unwrap();
                    match location.facing {
                        Cardinal::North => location.y += blocks_to_move,
                        Cardinal::East => location.x += blocks_to_move,
                        Cardinal::South => location.y -= blocks_to_move,
                        Cardinal::West => location.x -= blocks_to_move,
                    }
                }
            }
            character = chars.next();
        }

        return location.x.abs() + location.y.abs();
    }
}

pub mod part2 {
    use common::{Cardinal, Location};
    use std::collections::HashSet;

    pub fn blocks_away (directions: &String) -> i32 {
        let mut chars = directions.chars();
        let mut character = chars.next();
        let mut location = Location { x: 0, y: 0, facing: Cardinal::North};
        let mut visited = HashSet::new();

        'main: while character != None {
            let mut c: char = character.unwrap();
            if c == 'R' {
                match location.facing {
                    Cardinal::North => location.facing = Cardinal::East,
                    Cardinal::East => location.facing = Cardinal::South,
                    Cardinal::South => location.facing = Cardinal::West,
                    Cardinal::West => location.facing = Cardinal::North,
                }
            } else if c == 'L' {
                match location.facing {
                    Cardinal::North => location.facing = Cardinal::West,
                    Cardinal::East => location.facing = Cardinal::North,
                    Cardinal::South => location.facing = Cardinal::East,
                    Cardinal::West => location.facing = Cardinal::South,
                }
            } else {
                let mut number_of_blocks = String::new();
                loop {
                    match c.to_digit(10) {
                        Some(_) => number_of_blocks.push(c),
                        None => break,
                    }
                    character = chars.next();
                    match character {
                        Some(new_c) => c = new_c,
                        None => break,
                    }
                }
                if number_of_blocks.len() > 0 {
                    let blocks_to_move = number_of_blocks.parse::<i32>().unwrap();
                    let range = blocks_to_move + 1;
                    for _ in 1..range {
                        match location.facing {
                            Cardinal::North => location.y += 1,
                            Cardinal::East => location.x += 1,
                            Cardinal::South => location.y -= 1,
                            Cardinal::West => location.x -= 1,
                        }

                        let position = location.get_position_as_string();
                        println!("position: {}", position);
                        println!("lenght: {}", visited.len());
                        if visited.contains(&position) {
                            println!("Contains: {}", position);
                            break 'main;
                        }
                        visited.insert(position);
                    }
                }
            }
            character = chars.next();
        }

        return location.x.abs() + location.y.abs();
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn five_blocks_away() {
        let input = String::from("R2, L3");
        assert_eq!(5, part1::blocks_away(&input));
    }

    #[test]
    fn two_blocks_away() {
        let input = String::from("R2, R2, R2");
        assert_eq!(2, part1::blocks_away(&input));
    }

    #[test]
    fn twelve_blocks_away() {
        let input = String::from("R5, L5, R5, R3");
        assert_eq!(12, part1::blocks_away(&input));
    }

    #[test]
    fn twenty_six_blocks_away() {
        let input = String::from("R5, R21");
        assert_eq!(26, part1::blocks_away(&input));
    }

    #[test]
    fn first_location_visited_twice() {
        let input = String::from("R8, R4, R4, R8");
        assert_eq!(4, part2::blocks_away(&input));
    }

    #[test]
    fn first_location_visited_twice_one_blocks() {
        let input = String::from("R2, R4, R1, R20");
        assert_eq!(1, part2::blocks_away(&input));
    }

}
