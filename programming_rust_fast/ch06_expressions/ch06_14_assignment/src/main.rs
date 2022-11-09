fn main() {
    assignment();
}

/// 赋值运算符：
/// = += *= -=
/// - Rust 不支持链式赋值 a = b = 1
/// - Rust 没有自增/自减运算符 ++ --
fn assignment() {
    let mut a = 10;
    let b = 2;
    a += b;
    println!("a += b = {}", a);
    a -= b;
    println!("a -= b = {}", a);
    a *= b;
    println!("a *= b = {}", a);
    a /= b;
    println!("a /= b = {}", a);
}