use std::io;

const ASCII_TABLE_LENGTH: usize = 127;

const fn get_cipher_index(index: u8) -> u8 {
    let mut character_index: u8 = index;
    let mut count = 0;

    while count < 13 {
        character_index = character_index + 1;

        if character_index > 127 {
            character_index = 0;
        }

        count = count + 1;
    }

    character_index
}

const fn generate_cipher_table() -> [u8; ASCII_TABLE_LENGTH]  {
    let mut table: [u8; ASCII_TABLE_LENGTH] = [0; ASCII_TABLE_LENGTH];
    let mut i = 0;

    while i < ASCII_TABLE_LENGTH {
        table[i] = get_cipher_index(i as u8);
        i = i + 1;
    }

    table
}

const ASCII_TABLE: [u8; ASCII_TABLE_LENGTH] = generate_cipher_table();

fn main() {
    for line_result in std::io::stdin().lines() {
        if let Err(line) = line_result {
            eprintln!("Could not read from stdin");
            return;
        }

        if let Ok(line) = line_result {
            for character in line.chars() {
                print!("{}", ASCII_TABLE[character as usize] as char);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{get_cipher_index};

    #[test]
    fn test_cipher_character() {
        assert_eq!(110, get_cipher_index(97));
    }

    #[test]
    fn test_handle_overflow() {
        assert_eq!(7, get_cipher_index(122));
    }
}
