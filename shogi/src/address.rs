use super::common;

#[derive(Default)]
pub struct Address {
    pub column: u8,
    pub row: u8,
}

impl Address {
    pub fn new_from_number(index: u8) -> Self {
        Self {
            column: index % 11,
            row: index / 11,
        }
    }

    pub fn new_from_string(string: &str) -> Self {
        Self {
            column: string.as_bytes()[0] - 48,
            row: string.as_bytes()[1] - 96,
        }
    }

    pub fn to_index(&self) -> u8 {
        return self.row * 11 + self.column;
    }

    pub fn to_string(&self) -> String {
        let mut string = String::new();
        let column = common::integer_to_char(self.column);
        let mut row = common::integer_to_char(self.row);
        row = common::number_to_alphabet(row);
        string.push(column);
        string.push(row);
        return string
    }

}

pub fn index_to_address(index: u8) -> Address {
    return self::Address::new_from_number(index)
}

pub fn address_to_index(address: Address) -> u8 {
    return address.to_index()
}

pub fn index_to_row(index: u8) -> u8 {
    return index / 11
}

pub fn index_to_column(index: u8) -> u8 {
    return index % 11
}