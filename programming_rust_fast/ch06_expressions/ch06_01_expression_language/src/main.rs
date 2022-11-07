fn main() {

    expression_languages();

}

/// 表达式语言：
/// [Rust] 是表达式语言，表达式完成所有的工作，
/// [C] 语言中， [if] 和 [switch] 是语句，
/// 语句不产生值，
/// 它们不能用在表达式之间。
/// 在 [Rust] 中，[if] 和 [match] 语句可以产生值.
/// [match] 表达式可以产生值：
/// ```
/// println!("{} idea .",
///     match str.len() {
///         4 => "Good",
///         _ => "Bad" });
/// ```
/// - [if] 表达式产生值（用于对变量进行初始化）：
/// ```
/// let is_single_digit=
/// if num < 10 { true }
/// else { false };
/// ```
/// 这也解释了为什么 [Rust] 没有 [C] 语言中的[三目运算符]，
/// 因为使用[if]语句就可以满足了。
/// [C] 语言中的大多数控制流程都是[语句]，但是在 [Rust] 中都是[表达式]。
fn expression_languages() {
    let s = "good";
    match_expression(s);
    let num = 10;
    if_expression(num);
}

fn match_expression(str: &str) {
    println!("{} idea .",
         match str.len() {
             4 => "Good",
             _ => "Bad" })
}

fn if_expression(num: u8) {
    let is_single_digit=
        if num < 10 { true }
        else { false };
    println!("{} {} a single digit .",
        num,
        match is_single_digit {
            true => "is",
            false => "is not"
        }
    )
}
