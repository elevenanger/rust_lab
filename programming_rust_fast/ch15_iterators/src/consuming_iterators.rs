/// 简单累积运算
#[test]
fn test_simple_accumulation() {
    let a = [1, 2, 3];
    // 求和
    let sum: i32 = a.iter().by_ref().sum();
    assert_eq!(sum, 6);
    // 计数
    let count = a.iter().by_ref().filter(|x| *x % 2 == 0).count();
    assert_eq!(count, 1);
    // 求积
    let product: i32 = a.iter().by_ref().map(|x| x + 1).product();
    assert_eq!(product, 24);
}

/// max 和 min 方法比较的元素必须实现了 std::cmp::Ord trait
#[test]
fn test_min_and_max() {
    let a = [2, 3, 4, 5, -6];
    assert_eq!(a.iter().by_ref().max(), Some(&5));
    assert_eq!(a.iter().by_ref().min(), Some(&-6));
}