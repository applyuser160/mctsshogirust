#[cfg(test)]

mod tests {
    use crate::random::Random;

    #[test]
    fn case01() {
        let mut rand = Random::init();
        let n1 = rand.generate_one();
        assert!(n1 <= 9);
    }

    #[test]
    fn case02() {
        let mut rand = Random::new(5, 16);
        let n1 = rand.generate_one();
        assert!(n1 >= 5);
        assert!(n1 <= 16);
    }
}
