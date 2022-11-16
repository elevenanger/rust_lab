fn main() {
}

/// 生命周期参数结构：
/// <'elt> 为生命周期参数，
/// 表示对于任意指定的生命周期 'elt，
/// Extrema<'elt> 可以保存相同生命周期的引用。
struct Extrema<'elt> {
    greatest: &'elt i32,
    least: &'elt i32
}

/// 查找一个 i32 slice 中的最大值和最小值元素
fn find_extrema(slice: &[i32]) -> Extrema {
    let mut greatest = &slice[0];
    let mut least = &slice[0];

    for i in 1..slice.len() {
        if slice[i] > *greatest { greatest = &slice[i]; }
        if slice[i] < *least { least = &slice[i]; }
    }

    Extrema { greatest, least }
}

#[test]
fn test_find_extrema() {
    let a = [1, 32, 43, 12, -3, 23];
    let e = find_extrema(&a);

    assert_eq!(*e.least, -3);
    assert_eq!(*e.greatest, 43);
}