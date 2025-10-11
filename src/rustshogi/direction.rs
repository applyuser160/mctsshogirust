use strum_macros::EnumIter;

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, EnumIter, PartialEq, Eq)]
#[repr(usize)]
pub enum DirectionName {
    Up = 0,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
    DirectionNameNumber,
}

impl DirectionName {
    pub fn from_usize(n: usize) -> DirectionName {
        match n {
            0 => DirectionName::Up,
            1 => DirectionName::UpRight,
            2 => DirectionName::Right,
            3 => DirectionName::DownRight,
            4 => DirectionName::Down,
            5 => DirectionName::DownLeft,
            6 => DirectionName::Left,
            7 => DirectionName::UpLeft,
            _ => DirectionName::Up,
        }
    }
}
#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
pub struct Direction {
    pub vertical_vector: i8,
    pub horizon_vector: i8,
}

impl Direction {
    pub fn new(direction_name: DirectionName) -> Self {
        match direction_name {
            DirectionName::Up => Self {
                vertical_vector: -1,
                horizon_vector: 0,
            },
            DirectionName::UpRight => Self {
                vertical_vector: -1,
                horizon_vector: 1,
            },
            DirectionName::Right => Self {
                vertical_vector: 0,
                horizon_vector: 1,
            },
            DirectionName::DownRight => Self {
                vertical_vector: 1,
                horizon_vector: 1,
            },
            DirectionName::Down => Self {
                vertical_vector: 1,
                horizon_vector: 0,
            },
            DirectionName::DownLeft => Self {
                vertical_vector: 1,
                horizon_vector: -1,
            },
            DirectionName::Left => Self {
                vertical_vector: 0,
                horizon_vector: -1,
            },
            DirectionName::UpLeft => Self {
                vertical_vector: -1,
                horizon_vector: -1,
            },
            DirectionName::DirectionNameNumber => Self {
                vertical_vector: 0,
                horizon_vector: 0,
            },
        }
    }

    pub fn reverse(&mut self) {
        self.vertical_vector *= -1;
        self.horizon_vector *= -1;
    }
}
