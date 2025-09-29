#[cfg(test)]

mod tests {
    use crate::direction::{Direction, DirectionName};

    #[test]
    fn case01() {
        let dn = DirectionName::from_usize(3);
        assert_eq!(dn, DirectionName::DownLeft);
    }

    #[test]
    fn case02() {
        let direction = Direction::new(DirectionName::DownRight);
        assert_eq!(direction.name, DirectionName::DownRight);
        assert_eq!(direction.vertical_vector, 1);
        assert_eq!(direction.horizon_vector, -1);
    }

    #[test]
    fn case03() {
        let mut direction = Direction::new(DirectionName::UpLeft);
        direction.reverse();
        assert_eq!(direction.name, DirectionName::UpLeft);
        assert_eq!(direction.vertical_vector, 1);
        assert_eq!(direction.horizon_vector, -1);
    }
}
