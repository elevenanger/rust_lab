fn main() {
    closures();
}

/// 闭包：
/// 闭包是轻量化的类函数值，
/// 闭包通常包括参数列表，使用竖线包围，
/// 表达式和返回值类型
/// let name = |arg: type| -> return type {expr}
/// 调用闭包的语法和调用函数的语法基本一致。
fn closures() {
    let is_even = |x| x % 2 == 0;
    println!("is_even(2) => {}", is_even(2));
    println!("is_even(3) => {}", is_even(3));

    let product =
        |x: i32, y: i32| -> i32 {
            x * y
        };

    assert_eq!(product(2, 3), 6);
}
