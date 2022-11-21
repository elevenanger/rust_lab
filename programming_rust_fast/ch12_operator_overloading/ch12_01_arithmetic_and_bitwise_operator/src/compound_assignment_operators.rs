use std::ops::{RemAssign, BitXorAssign};
use crate::Complex;

/// 混合赋值运算符：
/// 算术运算符：
///
/// += -= *= /= %=
impl<T> RemAssign for Complex<T>
where T: RemAssign {
    fn rem_assign(&mut self, rhs: Self) {
        self.re %= rhs.re;
        self.im %= rhs.im;
    }
}

/// 位运算符：
///
/// &= |= ^= <<= >>=
impl<T> BitXorAssign for Complex<T>
where T: BitXorAssign {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.re ^= rhs.re;
        self.im ^= rhs.im;
    }
}

#[test]
fn test_compound_assignment_operator() {
    let mut c1 = Complex {re: 3, im: 2};
    let c2 = Complex {re: 2, im: 1};
    c1 %= c2;
    assert_eq!(c1, Complex {re: 1, im: 0});

    let c3 = Complex {re: 0, im: 1};
    c1 ^= c3;
    assert_eq!(c1, Complex {re: 1, im: 1});
}