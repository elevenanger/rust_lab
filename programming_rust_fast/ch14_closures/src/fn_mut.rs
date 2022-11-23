/// FnMut trait ：
/// 包含可变的数据或者引用的闭包
fn call_mut<F>(mut closure: F, times: i32)
    where F : FnMut() {
    for _ in 0..times {
        closure();
    }
}

#[test]
fn test_call_twice() {
    let mut i = 1;
    call_mut(|| {
        i += 1;
    }, 100);
    assert_eq!(i, 101)
}