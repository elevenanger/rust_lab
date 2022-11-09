fn main() {
    arithmetic_operators();

    bitwise_operator();

    comparison_operator();

    logical_operator();
}

/// 算数运算符：
/// Rust 的算数运算符包括 + - * / %,
/// - 整数溢出会引起 panic , a.wrapping_add(b) 用于未检查的算数运算
/// - 整数的除法舍入 0 a.checked_div(b) 返回 Option 值并且不会引起 panic
/// - 一元运算符 - 号对数字取负数，支持除了 unsigned 外其它的数值类型
/// - % 可以适用于浮点数值类型
fn arithmetic_operators() {
    let a = 10 as i32;
    let b = 2 as i32;
    println!("a + b = {}", a + b);
    println!("a - b = {}", a - b);
    println!("a * b = {}", a * b);
    println!("a / b = {}", a / b);
    println!("a % b = {}", a % b);

    println!("a.checked_div(0) = {:?}", a.checked_div(0));
    println!("a.wrapping_add(b) = {:?}", a.wrapping_add(b));

    println!("10.2 % 2.6 = {}", 10.2 % 2.6);
}

/// 位运算符：
/// | & ^ << >> !
/// Rust 使用 ! 取代 C 语言中按位否运算符 ~ ,
fn bitwise_operator() {
    let a = 0b0111;
    let b = 0b1110;
    println!("a | b = {:b}", a | b);
    println!("a & b = {:b}", a & b);
    println!("a ^ b = {:b}", a ^ b);
    println!("!a = {:b}", !a);
    println!("a << 1 = {:b}", a << 1);
    println!("b >> 1 = {:b}", b >> 1);
}

/// 比较运算符：
/// == != > >= < <=
/// rust 中的比较运算符的左右操作数必须是想同类型的数值
fn comparison_operator() {
    let a = 10;
    let b = 5;
    println!("a == b => {}", a == b);
    println!("a != b => {}", a != b);
    println!("a > b  => {}", a > b);
    println!("a >= b => {}", a >= b );
    println!("a < b  => {}", a < b);
    println!("a <= b => {}", a <= b);
}

/// 逻辑运算符：
/// Rust 中有两种短路逻辑运算符 && ||
/// 操作数都必须为 bool 类型
fn logical_operator() {
    let mut a = 2;
    let mut b = 1;
    if comp(&mut a, &mut b) && comp(&mut b, &mut a) {
        println!("a = {} b = {}", a, b);
    }

    if  comp(&mut b, &mut a) && comp(&mut a, &mut b) { }
    else {
        println!("a = {} b = {}", a, b);
    }

}

fn comp(a: &mut i32, b: &mut i32) -> bool {
    if a > b {
        *b += *a;
        true
    } else {
        *b += 1;
        *a += 1;
        false
    }
}