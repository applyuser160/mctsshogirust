#[cfg(test)]

mod tests {
    use crate::common::{
        alphabet_to_number, char_to_integer, integer_to_char, number_to_alphabet,
    };

    #[test]
    fn case01() {
        let num = alphabet_to_number('a');
        assert_eq!(num, '1');
    }

    #[test]
    fn case02() {
        let ch = number_to_alphabet('2');
        assert_eq!(ch, 'b');
    }

    #[test]
    fn case03() {
        let code = char_to_integer('3');
        assert_eq!(code, 3);
    }

    #[test]
    fn case04() {
        let ch = integer_to_char(8);
        assert_eq!(ch, '8');
    }
}
