mod iterator_and_into_iterator;
mod create_iterator;

/// 使用迭代器的 fold 方法，
/// ```
/// result = (0..=n).fold(0, |sum, i| sum + i)
/// ```
/// (o..=n) 是一个 RangeInclusive(i32) 值，
/// RangeInclusive(i32) 是一个产生整数的迭代器，
/// 从起始值 0 到 结束值 n ，inclusive 表示闭区间，包括 n 。
///
/// 迭代器的 fold() 函数，
/// 0, |sum, i| sum + i
/// 以 0 开始汇总，fold 接收 (0..n) 的每一个元素，
/// 将其应用在闭包中，
/// 闭包返回的值将会作为新的汇总值，
/// fold 方法最后返回的值就是 fold 函数本身的返回值，
/// 在这个例子中就是各个元素的和。
fn triangle(n: i32) -> i32 {
    (0..=n).fold(0, |sum, i| sum + i)
}

#[test]
fn test_triangle() {
    assert_eq!(6, triangle(3));
}