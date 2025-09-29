use pyo3::prelude::*;

#[allow(dead_code)]
#[pyclass(eq, eq_int)]
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum ColorType {
    None = -1,
    Black = 0,
    White,
    ColorNumber = 2,
}

impl ColorType {
    pub fn from_u8(n: u8) -> Self {
        match n {
            0 => Self::Black,
            1 => Self::White,
            2 => Self::ColorNumber,
            _ => Self::None,
        }
    }

    pub fn to_bool(&self) -> bool {
        self == &ColorType::White
    }
}

#[pymethods]
impl ColorType {
    #[new]
    pub fn new(value: u8) -> Self {
        Self::from_u8(value)
    }

    pub fn __ptr__(&self) -> String {
        format!("rustshogi.ColorType({})", *self as u8)
    }
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

impl std::fmt::Display for ColorType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", convert_string(*self))
    }
}
