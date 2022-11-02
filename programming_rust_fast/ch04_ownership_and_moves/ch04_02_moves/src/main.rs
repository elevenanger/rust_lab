fn main() {
    moves();

    more_operation();
}

/// Rust 中的 move 特性
/// Rust 对于大多数类型的赋值将值从源变量 move 到目标变量
/// 源变量会变成未初始化的状态
///
/// # Examples:
/// - move 特性
///
/// ```
/// let s = vec!["An".to_string(), "ger".to_string()];
/// let t = s;
/// let u = t;
/// ```
/// - s 初始化，具有 vector 的所有权
/// ```
/// let s = vec!["An".to_string(), "ger".to_string()];
/// ```
/// - 将 vector 的所有权 move 给 t , s 变成未初始化值
/// ```
/// let t = s;
/// ```
/// let u = t; 同理
///
/// 如果需要 s 还需要使用，可以使用 clone() 方法进行深拷贝，将拷贝的值赋值给其它的变量
/// ```
/// let t = s.clone();
/// ```
fn moves() {
    let s = vec!["An".to_string(), "ger".to_string()];
    /* 使用 clone 进行深拷贝 */
    let t = s.clone();
    let u = s.clone();

    println!("{:?}", u);
    println!("{:?}", s);
    println!("{:?}", t);
}

/// 更多的 move 操作
///
/// - 对于一个已赋值的变量再次进行赋值操作，原来的值将会被丢弃
/// ```
/// let mut s = "string1".to_string();
/// s = "string2".to_string();
/// ```
/// - 使用转移将值的所有权转移到另一个变量，两个值都会被保留
/// ```
/// s = "string3".to_string();
/// let s_2 = s;
/// s = "string4".to_string();
/// ```
///
fn more_operation() {
    /* s 赋值为 string1 */
    let mut s = "string1".to_string();
    /* s 赋值为 string2，原来的值 string1 被丢弃 */
    s = "string2".to_string();

    println!("s => {}", s);

    s = "string3".to_string();
    let s_2 = s; /* 将所有权 move 到 s_2 */
    s = "string4".to_string();
    println!("s => {} s_2 = > {}", s, s_2);
}