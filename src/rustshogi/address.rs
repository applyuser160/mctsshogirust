use super::common;

use pyo3::prelude::*;

#[derive(Default, PartialEq, Debug)]
#[pyclass]
pub struct Address {
    #[pyo3(get, set)]
    pub column: u8,
    #[pyo3(get, set)]
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
        Self { column: c, row: r }
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
        self.row * 11 + self.column
    }

    #[allow(dead_code)]
    pub fn to_string(&self) -> String {
        let mut string = String::new();
        let column = common::integer_to_char(self.column);
        let mut row = common::integer_to_char(self.row);
        row = common::number_to_alphabet(row);
        string.push(column);
        string.push(row);
        string
    }
}

#[pymethods]
impl Address {
    #[new]
    pub fn python_new(column: u8, row: u8) -> Self {
        Self { column, row }
    }

    pub fn __repr__(&self) -> String {
        format!("Address(column={}, row={})", self.column, self.row)
    }

    pub fn __str__(&self) -> String {
        format!("Address(column={}, row={})", self.column, self.row)
    }

    pub fn __eq__(&self, other: &Self) -> bool {
        self.column == other.column && self.row == other.row
    }

    pub fn __ne__(&self, other: &Self) -> bool {
        self.column != other.column || self.row != other.row
    }

    #[allow(dead_code)]
    #[pyo3(name = "to_int")]
    pub fn python_to_int(&self) -> usize {
        self.to_index() as usize
    }
}

#[allow(dead_code)]
pub fn index_to_row(index: u8) -> u8 {
    index / 11
}

#[allow(dead_code)]
pub fn index_to_column(index: u8) -> u8 {
    index % 11
}
