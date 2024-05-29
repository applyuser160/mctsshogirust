#[cfg(test)]

mod tests {
    use crate::random::Random;


    #[test]
    fn case01() {
        let mut rand = Random::init();
        let n1 = rand.generate_one();
        assert!(n1 > 0);
        assert!(n1 <= 9);
    }
    
}