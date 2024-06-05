use super::common;

#[derive(Default)]
#[derive(PartialEq)]
#[derive(Debug)]
pub struct Address {
    pub column: u8,
    pub row: u8,
}

impl Address {
    pub fn from_number(index: u8) -> Self {
        Self {
            column: index % 11,
            row: index / 11,
        }
    }

    #[allow(dead_code)]
    pub fn from_numbers(c: u8, r: u8) -> Self {
        Self {
            column: c,
            row: r,
        }
    }

    #[allow(dead_code)]
    pub fn from_string(string: &str) -> Self {
        Self {
            column: string.as_bytes()[0] - 48,
            row: string.as_bytes()[1] - 96,
        }
    }

    #[allow(dead_code)]
    pub fn to_index(&self) -> u8 {
        return self.row * 11 + self.column;
    }

    #[allow(dead_code)]
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

#[allow(dead_code)]
pub fn index_to_address(index: u8) -> Address {
    return self::Address::from_number(index)
}

#[allow(dead_code)]
pub fn address_to_index(address: Address) -> u8 {
    return address.to_index()
}

#[allow(dead_code)]
pub fn index_to_row(index: u8) -> u8 {
    return index / 11
}

#[allow(dead_code)]
pub fn index_to_column(index: u8) -> u8 {
    return index % 11
}