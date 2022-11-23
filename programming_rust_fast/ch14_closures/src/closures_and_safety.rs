/// 对于一些闭包,
/// 比如 drop() 某个值，
/// 只能调用一次，
/// 再次调用就会导致 double free ，
/// 为了防止这种情况发生，
/// 对于这种类型的闭包，
/// 可以实现 FnOnce trait ,
/// 一旦调用了 FnOnce trait 闭包，
/// 这个闭包本身就会被使用掉,
/// Rust 会根据闭包中的代码自动推断出闭包的类型是不是 FnOnce
fn call_once<F>(closure: F)
where F: FnOnce() {
    closure();
}

#[test]
fn test_kill_by_once() {
    let a = 10;
    call_once(||drop(a));
}