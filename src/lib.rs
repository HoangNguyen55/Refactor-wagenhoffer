pub fn gcd(mut m: i32, mut n: i32) -> i32 {
    while m % n != 0 {
        let temp = m;
        m = n;
        n = temp % n;
    }
    n
}

pub fn int_to_fraction(x: i32) -> Fraction {
    Fraction { num: x, den: 1 }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Fraction {
    num: i32,
    den: i32,
}

impl Fraction {
    pub fn new(top: i32, bottom: i32) -> Self {
        let cmmn = gcd(top, bottom);
        Fraction {
            num: top / cmmn,
            den: bottom / cmmn,
        }
    }

    pub fn eq(&self, other_fraction: &Fraction) -> bool {
        let first_num = self.num * other_fraction.den;
        let second_num = other_fraction.num * self.den;
        first_num == second_num
    }

    pub fn add(&self, other_fraction: &Fraction) -> Fraction {
        let new_num = self.num * other_fraction.den + self.den * other_fraction.num;
        let new_den = self.den * other_fraction.den;
        Fraction::new(new_num, new_den)
    }

    pub fn sub(&self, other_fraction: &Fraction) -> Fraction {
        let new_num = self.num * other_fraction.den - self.den * other_fraction.num;
        let new_den = self.den * other_fraction.den;
        Fraction::new(new_num, new_den)
    }

    pub fn mul(&self, other_fraction: &Fraction) -> Fraction {
        let new_num = self.num * other_fraction.num;
        let new_den = self.den * other_fraction.den;
        Fraction::new(new_num, new_den)
    }

    pub fn div(&self, other_fraction: &Fraction) -> Fraction {
        let new_num = self.num * other_fraction.den;
        let new_den = self.den * other_fraction.num;
        Fraction::new(new_num, new_den)
    }

    pub fn gt(&self, other_fraction: &Fraction) -> bool {
        self.num * other_fraction.den > other_fraction.num * self.den
    }

    pub fn ge(&self, other_fraction: &Fraction) -> bool {
        self.num * other_fraction.den >= other_fraction.num * self.den
    }

    pub fn lt(&self, other_fraction: &Fraction) -> bool {
        self.num * other_fraction.den < other_fraction.num * self.den
    }

    pub fn le(&self, other_fraction: &Fraction) -> bool {
        self.num * other_fraction.den <= other_fraction.num * self.den
    }

    pub fn ne(&self, other_fraction: &Fraction) -> bool {
        let first_num = self.num * other_fraction.den;
        let second_num = other_fraction.num * self.den;
        first_num != second_num
    }

    pub fn repr(&self) -> String {
        format!("{}/{}", self.num, self.den)
    }

    pub fn get_num(&self) -> i32 {
        self.num
    }

    pub fn get_den(&self) -> i32 {
        self.den
    }

    pub fn show(&self) {
        println!("{}", self);
    }
}

impl std::fmt::Display for Fraction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}/{}", self.num, self.den)
    }
}

// pub fn main() {
//     let x = Fraction::new(1, 2);
//     println!("{}", x);
//     let y = Fraction::new(2, 3);
//     println!("{}", y);
//     assert_eq!(y, Fraction::new(2, 3));
//     println!("{:?}", x.add(&y));
//     assert_eq!(x.add(&y), Fraction::new(7, 6));
//     println!("{}", x.eq(&y));
// }
