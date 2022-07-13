use std::io;
use std::process;

const ASCII_TABLE_LENGTH: usize = 127;
const ASCII_TABLE: [u8; ASCII_TABLE_LENGTH] = generate_cipher_table();

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

fn main() {
    for line_result in std::io::stdin().lines() {
        match line_result {
            Ok(line) => {
                for character in line.chars() {
                    if (character as usize) > 127 || (character as usize) < 0 {
                        eprintln!("Invalid character: {}", character);
                        process::exit(1);
                    }

                    print!("{}", ASCII_TABLE[character as usize] as char);
                }
            },
            Err(_) => {
                eprintln!("Could not read from stdin");
                process::exit(1);
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
