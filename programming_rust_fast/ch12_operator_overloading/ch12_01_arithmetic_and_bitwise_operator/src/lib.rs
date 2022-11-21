mod unary_operator;
mod binary_operator;
mod compound_assignment_operators;
mod equivalence_comparisons;
mod ordered_comparisons;
mod index_mut;

use std::ops::Add;

#[test]
fn test_add() {
    // a + b 和 a.add(b) 是一样的，
    // a 和 b 是实现了 std::ops::Add 的相同类型，
    // 这种做法称之为算术运算符重载,
    assert_eq!(10.0.add(12.0), 10.0 + 12.0);
    assert_eq!(1.add(2), 1 + 2);
}

#[derive(Debug)]
pub struct Complex<T> {
    re: T,
    im: T,
}

/// 可以为自定义的类型实现 Add trait 实现算术运算符的重载。
impl<T> Add for Complex<T>
where T: Add<Output = T>{
    type Output = Complex<T>;

    fn add(self, rhs: Self) -> Self {
        Complex {
            re: self.re + rhs.re,
            im: self.im + rhs.im
        }
    }
}

#[test]
fn test_complex_add() {
    let c1: Complex<i32> = Complex {re: 10, im: 10};
    let c2: Complex<i32> = Complex {re: 9, im: 9};

    assert_eq!(c1 + c2, Complex::<i32> {re: 19, im: 19});
}