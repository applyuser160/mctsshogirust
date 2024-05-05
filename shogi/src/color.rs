#[allow(dead_code)]
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum ColorType {
    None = -1, Black = 0, White, ColorNumber = 2,
}

#[allow(dead_code)]
pub fn get_reverse_color(color_type: ColorType) -> ColorType {
    match color_type {
        ColorType::Black => ColorType::White,
        ColorType::White => ColorType::Black,
        _ => ColorType::None,
    }
}

#[allow(dead_code)]
pub fn convert_string(color_type: ColorType) -> char {
    match color_type {
        ColorType::Black => 'b',
        ColorType::White => 'w',
        _ => 'd',
    }
}

#[allow(dead_code)]
pub fn convert_from_string(character: char) -> ColorType {
    match character {
        'b' => ColorType::Black,
        'w' => ColorType::White,
        _ => ColorType::None,
    }
}