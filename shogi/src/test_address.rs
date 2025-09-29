#[cfg(test)]

mod tests {
    use crate::address::{
        address_to_index, index_to_address, index_to_column, index_to_row, Address,
    };

    #[test]
    fn case01() {
        let address = Address::from_number(57);
        assert_eq!(address.column, 2);
        assert_eq!(address.row, 5);
    }

    #[test]
    fn case02() {
        let address = Address::from_numbers(5, 9);
        assert_eq!(address.column, 5);
        assert_eq!(address.row, 9);
    }

    #[test]
    fn case03() {
        let address = Address::from_string("9i");
        assert_eq!(address.column, 9);
        assert_eq!(address.row, 9);
    }

    #[test]
    fn case04() {
        let address = Address::from_string("9i");
        let index = address.to_index();
        assert_eq!(index, 108);
    }

    #[test]
    fn case05() {
        let address = Address::from_string("9i");
        let str = address.to_string();
        assert_eq!(str, String::from("9i"))
    }

    #[test]
    fn case06() {
        let address = index_to_address(100);
        assert_eq!(address.column, 1);
        assert_eq!(address.row, 9);
    }

    #[test]
    fn case07() {
        let address = index_to_address(100);
        let index = address_to_index(address);
        assert_eq!(index, 100);
    }

    #[test]
    fn case08() {
        let row = index_to_row(100);
        assert_eq!(row, 9);
    }

    #[test]
    fn case09() {
        let row = index_to_column(100);
        assert_eq!(row, 1);
    }
}
