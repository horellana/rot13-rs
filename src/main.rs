use std::process;

const fn get_lowercase_cipher_index(index: u8) -> u8 {
    let mut character_index: u8 = index;
    let mut count = 0;

    while count < 13 {
        character_index = character_index + 1;

        if character_index > 122 {
            character_index = 97;
        }

        count = count + 1;
    }

    character_index
}

const fn get_uppercase_cipher_index(index: u8) -> u8 {
    let mut character_index: u8 = index;
    let mut count = 0;

    while count < 13 {
        character_index = character_index + 1;

        if character_index > 90 {
            character_index = 65;
        }

        count = count + 1;
    }

    character_index
}

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

    let mut start: usize = 0;
    let mut end: usize = 127;
    let mut i: usize = start;

    while i < end {
        table[i] = i as u8;
        i = i + 1;
    }

    start = 65;
    end = 90;
    i = start;

    while i <= end {
        table[i] = get_uppercase_cipher_index(i as u8);
        i = i + 1;
    }

    start = 97;
    end = 122;
    i = start;

    while i <= end {
        table[i] = get_lowercase_cipher_index(i as u8);
        i = i + 1;
    }

    table
}

const ASCII_TABLE_LENGTH: usize = 127;
const ASCII_TABLE: [u8; ASCII_TABLE_LENGTH] = generate_cipher_table();

fn rot13(message: &str) -> String {
    String::from(message
                 .chars()
                 .map(|character| {
                     if (character as usize) < 127 {
                         ASCII_TABLE[character as usize] as char
                     }
                     else {
                         character
                     }

                 })
                 .collect::<String>())
}

fn main() {
    for line_result in std::io::stdin().lines() {
        match line_result {
            Ok(line) => {
                for character in line.chars() {
                    if ((character as i8) > 127) || ((character as i8) < 0) {
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
    use crate::{rot13, get_cipher_index};

    #[test]
    fn test_cipher_character() {
        assert_eq!(110, get_cipher_index(97));
        assert_eq!(rot13("z"), "m");
    }

    #[test]
    fn test_handle_overflow() {
        assert_eq!(7, get_cipher_index(122));
    }

    #[test]
    fn test_rot13() {
        assert_eq!(rot13("test"), "grfg");
        assert_eq!(rot13("Test"), "Grfg");
        assert_eq!(rot13("abcdefghijklmnopqrstuvwxyz"), "nopqrstuvwxyzabcdefghijklm");
    }
}
