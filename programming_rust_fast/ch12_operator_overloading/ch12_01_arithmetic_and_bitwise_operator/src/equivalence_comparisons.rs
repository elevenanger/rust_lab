use std::cmp::PartialEq;
use crate::Complex;

/// 判等运算符：
/// != ==
///
/// 部分判等运算，
/// 不能保障 T 始终等于 T,
/// 可以自己实现 PartialEq 中的 eq 和 ne
/// 但是必须确保两个方法之间是互补的关系。
impl<T> PartialEq for Complex<T>
where T: PartialEq {
    fn eq(&self, other: &Self) -> bool {
        return self.re == other.re && self.im == other.im;
    }

    fn ne(&self, other: &Self) -> bool {
        return self.re != other.re || self.im != other.im;
    }
}

#[test]
fn test_partial_eq() {
    let c1 = Complex {re: 1, im: 0};
    let c2 = Complex {re: 0, im: 1};
    assert_ne!(c1, c2);
    assert!(!(c1 == c2));
}