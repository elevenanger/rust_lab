use std::ops::{Neg, Not};
use crate::Complex;

/// 一元运算符 - :
impl<T> Neg for Complex<T>
where T: Neg<Output = T> {
    type Output = Complex<T>;

    fn neg(self) -> Self::Output {
        Complex {
            re: -self.re,
            im: -self.im
        }
    }
}

/// 一元运算符 ! :
impl <T> Not for Complex<T>
where T: Not<Output = T> {
    type Output = Complex<T>;

    fn not(self) -> Self::Output {
        Complex {
            re: !self.re,
            im: !self.im
        }
    }
}

#[test]
fn test_neg_complex() {
    let c = Complex {
        re: 1,
        im: 1
    };
    assert_eq!(-c, Complex {re: -1, im: -1});
}

#[test]
fn test_not_complex() {
    let c = Complex {
        re: 1,
        im: 1
    };
    assert_eq!(!c, Complex {re: -2, im: -2});
}