#[cfg(test)]
mod tests {
    use crate::direction::{Direction, DirectionName};

    #[test]
    fn test_direction_new() {
        let direction = Direction::new(DirectionName::Up);
        assert_eq!(direction.vertical_vector, -1);
        assert_eq!(direction.horizon_vector, 0);
    }

    #[test]
    fn test_direction_from_usize() {
        let direction_name = DirectionName::from_usize(3);
        let direction = Direction::new(direction_name);
        assert_eq!(direction.vertical_vector, 1);
        assert_eq!(direction.horizon_vector, 1);
    }

    #[test]
    fn test_direction_reverse() {
        let mut direction = Direction::new(DirectionName::UpLeft);
        direction.reverse();
        assert_eq!(direction.vertical_vector, 1);
        assert_eq!(direction.horizon_vector, 1);
    }
}
