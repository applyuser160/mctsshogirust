#[allow(dead_code)]
pub fn alphabet_to_number(alphabet: char) -> char {
    return ((alphabet as u8) - 48) as char;
}

pub fn number_to_alphabet(number: char) -> char {
    return ((number as u8) + 48) as char;
}

#[allow(dead_code)]
pub fn char_to_integer(character: char) -> u8 {
    return (character as u8) - 48;
}

pub fn integer_to_char(integer: u8) -> char {
    return (integer + 48) as char;
}