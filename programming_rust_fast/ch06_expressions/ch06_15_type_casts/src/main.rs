fn main() {
    cast();
}

/// 类型转换：
/// 使用 as 关键字，将一个值的类型转换成另一个类型，
/// 允许进行转换的类型：
/// - 数值可以转换成任意内置数值类型
/// - bool 、 char 或者枚举类型可以转换成任意整数类型
/// - 设计某些不安全的指针类型的转换也是允许的
fn cast() {
    integer_cast();

    float_point_cast();

    bool_cast_to_integer();

    char_cast_to_integer();
}

/// 整数类型转换：
/// - 将整数类型转换成较小的类型将会使用截取的方式进行转换
/// - 有符号类型转换成更大的类型是有符号扩展的
/// - 无符号类型转换成较大类型是按 0 扩展的
fn integer_cast() {
    let a: i64 = 100_000_000_000;
    let b = -100;
    let c = 100;

    println!("a as i32 => {:b}", a as i32);
    println!("b as i64 => {}", b as i64);
    println!("c as i64 => {}", c as i64);
}

/// 浮点类型转换：
/// - 浮点类型转换为整数类型是按 0 舍入的
fn float_point_cast() {
    let a = -1.8;
    println!("a as i32 => {}", a as i32);
}

/// bool 类型转换成整数类型
fn bool_cast_to_integer() {
    println!("true as i32 => {} ", true as i32);
    println!("false as i32 => {}", false as i32)
}

/// char 类型和整数类型之间的转换：
fn char_cast_to_integer() {
    println!("A as i32 = {}", 'A' as i32);

    let c = std::char::from_u32(100);
    println!("c = {:?}", c);
}

