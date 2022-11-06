fn main() {
    borrowing_local_variable();

    receiving_references_as_function_arguments();
}

/// 借用局部变量：
/// - 约束1：对变量的引用不能超过变量本身的生命周期（最大生命周期）,
/// 变量的生命周期必须包括借用该变量的引用。
/// - 约束2：引用的生命周期必须包含它所赋值的变量的生命周期（最小生命周期）。
/// ```
/// let v = vec![1, 2, 3];
/// {
///     let r = &v[1];
///     assert_eq!(r, 2);
/// }
/// println!("{:?}", v);
/// ```
/// v 的生命周期到 println! 宏结束；
/// r 的声明周期在 assert_eq! 调用结束；
/// 对变量 v[1] 引用 &v[1] 的生命周期和 v 的一样；
/// r 是 &v[1] 赋值的变量；
/// v 的生命周期包含 &v[1];
/// &v[1] 的生命周期包含 r 的生命周期；
fn borrowing_local_variable() {
    let v = vec![1, 2, 3];
    {
        let r = &v[1];
        assert_eq!(r, 2);
    }
    println!("{:?}", v);
}

/// 接收引用作为函数的实参：
fn receiving_references_as_function_arguments() {

}