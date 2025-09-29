#[cfg(test)]

mod tests {
    use crate::color::{
        convert_from_string, convert_string, get_reverse_color, ColorType,
    };

    #[test]
    fn case01() {
        let color = ColorType::from_u8(0);
        assert_eq!(color as u8, 0);
    }

    #[test]
    fn case02() {
        let color = ColorType::from_u8(1);
        assert_eq!(color.to_bool(), true);
    }

    #[test]
    fn case03() {
        let color = get_reverse_color(ColorType::Black);
        assert_eq!(color as u8, ColorType::White as u8);
    }

    #[test]
    fn case04() {
        let color = ColorType::from_u8(0);
        assert_eq!(convert_string(color), 'b');
    }

    #[test]
    fn case05() {
        let color = convert_from_string('b');
        assert_eq!(color as u8, ColorType::Black as u8);
    }
}
