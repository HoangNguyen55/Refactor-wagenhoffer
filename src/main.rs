fn gcd(mut m: i32, mut n: i32) -> i32 {
    while m % n != 0 {
        let temp = m;
        m = n;
        n = temp % n;
    }
    n
}

fn int_to_fraction(x: i32) -> Fraction {
    Fraction { num: x, den: 1 }
}

#[derive(Debug, PartialEq, Eq)]
struct Fraction {
    num: i32,
    den: i32,
}

impl Fraction {
    fn new(top: i32, bottom: i32) -> Self {
        let cmmn = gcd(top, bottom);
        Fraction {
            num: top / cmmn,
            den: bottom / cmmn,
        }
    }

    fn eq(&self, other_fraction: &Fraction) -> bool {
        let first_num = self.num * other_fraction.den;
        let second_num = other_fraction.num * self.den;
        first_num == second_num
    }

    fn add(&self, other_fraction: &Fraction) -> Fraction {
        let new_num = self.num * other_fraction.den + self.den * other_fraction.num;
        let new_den = self.den * other_fraction.den;
        Fraction::new(new_num, new_den)
    }

    fn sub(&self, other_fraction: &Fraction) -> Fraction {
        let new_num = self.num * other_fraction.den - self.den * other_fraction.num;
        let new_den = self.den * other_fraction.den;
        Fraction::new(new_num, new_den)
    }

    fn mul(&self, other_fraction: &Fraction) -> Fraction {
        let new_num = self.num * other_fraction.num;
        let new_den = self.den * other_fraction.den;
        Fraction::new(new_num, new_den)
    }

    fn div(&self, other_fraction: &Fraction) -> Fraction {
        let new_num = self.num * other_fraction.den;
        let new_den = self.den * other_fraction.num;
        Fraction::new(new_num, new_den)
    }

    fn gt(&self, other_fraction: &Fraction) -> bool {
        self.num * other_fraction.den > other_fraction.num * self.den
    }

    fn ge(&self, other_fraction: &Fraction) -> bool {
        self.num * other_fraction.den >= other_fraction.num * self.den
    }

    fn lt(&self, other_fraction: &Fraction) -> bool {
        self.num * other_fraction.den < other_fraction.num * self.den
    }

    fn le(&self, other_fraction: &Fraction) -> bool {
        self.num * other_fraction.den <= other_fraction.num * self.den
    }

    fn ne(&self, other_fraction: &Fraction) -> bool {
        let first_num = self.num * other_fraction.den;
        let second_num = other_fraction.num * self.den;
        first_num != second_num
    }

    fn repr(&self) -> String {
        format!("{}/{}", self.num, self.den)
    }

    fn get_num(&self) -> i32 {
        self.num
    }

    fn get_den(&self) -> i32 {
        self.den
    }

    fn show(&self) {
        println!("{}", self);
    }
}

impl std::fmt::Display for Fraction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}/{}", self.num, self.den)
    }
}

fn main() {
    let x = Fraction::new(1, 2);
    println!("{}", x);
    let y = Fraction::new(2, 3);
    println!("{}", y);
    assert_eq!(y, Fraction::new(2, 3));
    println!("{:?}", x.add(&y));
    assert_eq!(x.add(&y), Fraction::new(7, 6));
    println!("{}", x.eq(&y));
}

#[cfg(test)]
mod tests {
    use super::*;

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
