use std::ops::BitAnd;
use std::ops::Shl;
use crate::Complex;

/// 二元运算符：
/// + - * / %
///
/// & | ^ << >>
impl<T> BitAnd for Complex<T>
where T: BitAnd<Output = T> {
    type Output = Complex<T>;

    fn bitand(self, rhs: Self) -> Self::Output {
        Complex {
            im: self.im & rhs.im,
            re: self.re & rhs.re
        }
    }
}

impl<T> Shl for Complex<T>
where T: Shl<Output = T> {
    type Output = Complex<T>;

    fn shl(self, rhs: Self) -> Self::Output {
        Complex {
            im: self.im << rhs.im,
            re: self.re << rhs.re
        }
    }
}

#[test]
fn test_complex_bitand() {
    let c1 = Complex {im: 1, re: 0};
    let c2 = Complex {im: 0, re: 1};

    assert_eq!(c1 & c2, Complex{im: 0, re: 0});
}

#[test]
fn test_complex_shl() {
    let c1 = Complex {im: 1, re: 1};
    let c2 = Complex {im: 2, re: 2};

    assert_eq!(c1 << c2, Complex{im: 4, re: 4});
}