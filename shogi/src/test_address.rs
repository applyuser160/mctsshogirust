#[cfg(test)]

mod tests {
    use crate::address::{
        address_to_index, index_to_address, index_to_column, index_to_row, Address,
    };

    #[test]
    fn test_address_from_number() {
        let address = Address::from_number(57);
        assert_eq!(address.column, 2);
        assert_eq!(address.row, 5);
    }

    #[test]
    fn test_address_from_numbers() {
        let address = Address::from_numbers(5, 9);
        assert_eq!(address.column, 5);
        assert_eq!(address.row, 9);
    }

    #[test]
    fn test_address_from_string() {
        let address = Address::from_string("9i");
        assert_eq!(address.column, 9);
        assert_eq!(address.row, 9);
    }

    #[test]
    fn test_address_to_index() {
        let address = Address::from_string("9i");
        let index = address.to_index();
        assert_eq!(index, 108);
    }

    #[test]
    fn test_address_to_string() {
        let address = Address::from_string("9i");
        let str = address.to_string();
        assert_eq!(str, String::from("9i"))
    }

    #[test]
    fn test_address_index_to_address() {
        let address = index_to_address(100);
        assert_eq!(address.column, 1);
        assert_eq!(address.row, 9);
    }

    #[test]
    fn test_address_address_to_index() {
        let address = index_to_address(100);
        let index = address_to_index(address);
        assert_eq!(index, 100);
    }

    #[test]
    fn test_address_index_to_row() {
        let row = index_to_row(100);
        assert_eq!(row, 9);
    }

    #[test]
    fn test_address_index_to_column() {
        let row = index_to_column(100);
        assert_eq!(row, 1);
    }
}
