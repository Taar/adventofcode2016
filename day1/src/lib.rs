enum Cardinal {
    North,
    East,
    South,
    West,
}

struct Location {
    x: i32,
    y: i32,
    facing: Cardinal,
}

pub fn blocks_away (directions: String) -> i32 {
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

    if location.x < 0 {
        location.x *= -1
    }

    if location.y < 0 {
        location.y *= -1
    }

    return location.x + location.y;
}


#[cfg(test)]
mod tests {
    use super::blocks_away;

    #[test]
    fn five_blocks_away() {
        let input = String::from("R2, L3");
        assert_eq!(5, blocks_away(input));
    }

    #[test]
    fn two_blocks_away() {
        let input = String::from("R2, R2, R2");
        assert_eq!(2, blocks_away(input));
    }

    #[test]
    fn twelve_blocks_away() {
        let input = String::from("R5, L5, R5, R3");
        assert_eq!(12, blocks_away(input));
    }

    #[test]
    fn twenty_six_blocks_away() {
        let input = String::from("R5, R21");
        assert_eq!(26, blocks_away(input));
    }
}
