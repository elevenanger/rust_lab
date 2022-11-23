/// 非 move ，
/// 只拥有共享引用的闭包，
/// 该引用是 Clone 和 Copy 类型，
/// 所以对应的闭包也是 Clone 和 Copy 类型,
///
/// 拥有可变引用的闭包，
/// 因为可变引用既不是 Copy 也不是 Clone 类型，
/// 所以对应的闭包也既非 Copy 也非 Clone 类型，
///
#[test]
fn test_copy_and_clone_closures() {
    let y = 10;
    let add_y = |x| x + y;
    let copy_add_y = add_y;
    assert_eq!(11, copy_add_y(1));
    let clone_add_y = add_y.clone();
    assert_eq!(12, clone_add_y(2));

}

#[test]
fn test_mut_closures() {
    let mut y = 10;
    let mut y_add_x = |x| y += x;
    y_add_x(1);
    assert_eq!(y, 11);
}

/// 对于 move 闭包，
/// 如果捕获的值都是 Copy 类型，闭包就是 Copy 类型，
/// 如果捕获的值都是 Clone 类型，闭包就是 Clone 类型，
/// 需要将一个闭包作为参数传给多个函数的时候这个特性非常有用。
#[test]
fn test_move_closure() {
    let mut s = "Hello, ".to_string();
    let greeting = move |name| {
        s.push_str(name);
        println!("{}", s);
    };
    let mut g = greeting.clone();
    g("Liu");
    greeting.clone()("An");
    greeting.clone()("Lin");
}