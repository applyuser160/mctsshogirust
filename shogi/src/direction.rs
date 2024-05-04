#[allow(dead_code)]
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum DirectionName {
    Up, UpLeft, Left, DownLeft, Down, DownRight, Right, UpRight,
    DirectionNameNumber = 8
}

#[derive(Clone)]
pub struct Direction {

    #[allow(dead_code)]
    pub name: DirectionName,

    #[allow(dead_code)]
    pub vertical_vector: i8,

    #[allow(dead_code)]
    pub horizon_vector: i8,
}

#[allow(dead_code)]
impl Direction {
    pub fn new(name: DirectionName) -> Self {
        let mut vertical_vector: i8 = 0;
        let mut horizon_vector: i8 = 0;
        if name == DirectionName::Up {
            vertical_vector = -1;
            horizon_vector = 0;
        } else if name == DirectionName::UpLeft {
            vertical_vector = -1;
            horizon_vector = 1;
        } else if name == DirectionName::Left {
            vertical_vector = 0;
            horizon_vector = 1;
        } else if name == DirectionName::DownLeft {
            vertical_vector = 1;
            horizon_vector = 1;
        } else if name == DirectionName::Down {
            vertical_vector = 1;
            horizon_vector = 0;
        } else if name == DirectionName::DownRight {
            vertical_vector = 1;
            horizon_vector = -1;
        } else if name == DirectionName::Right {
            vertical_vector = 0;
            horizon_vector = -1;
        } else if name == DirectionName::UpRight {
            vertical_vector = -1;
            horizon_vector = -1;
        }
        Self {
            name: name,
            vertical_vector: vertical_vector,
            horizon_vector: horizon_vector,
        }
    }

    pub fn reverse(&self) -> Self {
        let mut copy = self.clone();
        copy.vertical_vector *= -1;
        copy.horizon_vector *= -1;
        return copy
    }

}