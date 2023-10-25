#[cfg(test)]
mod tests {
    // mod refactor_wagenhoffer;
    use refactor_wagenhoffer::*;

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(48, 18), 6);
        assert_eq!(gcd(17, 5), 1);
        assert_eq!(gcd(60, 48), 12);
    }

    #[test]
    fn test_int_to_fraction() {
        assert_eq!(int_to_fraction(5), Fraction::new(5, 1));
        assert_eq!(int_to_fraction(3), Fraction::new(3, 1));
        assert_eq!(int_to_fraction(-2), Fraction::new(-2, 1));
    }

    #[test]
    fn test_fraction_eq() {
        let f1 = Fraction::new(1, 2);
        let f2 = Fraction::new(2, 3);
        assert_eq!(f1.eq(&f2), false);
    }

    #[test]
    fn test_fraction_add() {
        let f1 = Fraction::new(1, 2);
        let f2 = Fraction::new(2, 3);
        assert_eq!(f1.add(&f2), Fraction::new(7, 6));
    }
}
