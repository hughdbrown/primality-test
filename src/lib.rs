pub fn is_probably_prime(n: usize) -> bool {
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prime_2() {
        let result = is_probably_prime(2);
        assert!(result);
    }

    #[test]
    fn prime_3() {
        let result = is_probably_prime(3);
        assert!(result);
    }

    #[test]
    fn prime_4() {
        let result = is_probably_prime(4);
        assert!(! result);
    }


}
