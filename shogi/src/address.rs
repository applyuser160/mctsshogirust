#[derive(Default)]
pub struct Address {
    pub column: u8,
    pub row: u8,
}

impl Address {
    pub fn new(index: u8) -> Self {
        Self {
            column: index % 11,
            row: index / 11,
        }
    }
}