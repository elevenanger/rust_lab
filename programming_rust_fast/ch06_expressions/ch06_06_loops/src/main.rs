fn main() {
    loops();
}

/// 循环表达式：
/// - while 循环
/// ```
/// while condition {
///     block
/// }
/// ```
/// - while let 循环
/// ```
/// while let  pattern = expr {
///     block
/// }
/// ```
/// - loop 循环
/// ```
/// loop {
///     block
/// }
/// ```
/// - for 循环
/// ```
/// for pattern in iterable {
///     block
/// }
/// ```
/// 在 [Rust] 中，循环也是表达式，
/// 但是还是 [while] 和 [for] 循环的值永远都是 [()] ,
/// 所以它们的值不是很有用，
/// [loop] 表达式可以产生值。
fn loops() {
    let sen = "Keep Simple";

    while_expression(sen);

    while_let_expression(sen);

    loop_expression(sen);

    for_expression(sen);
}

fn while_expression(sentence: &str) {
    let mut count = 5;
    while  count > 0 {
        println!("{}", sentence);
        count -= 1;
    }
    println!();
}

fn while_let_expression(sentence: &str) {
    let mut chars= sentence.chars();
    while let Some(c) = chars.next() {
        print!("{}", c.to_uppercase());
    }
    println!()
}

/// loop 循环：
/// [loop] 循环创建一个无限循环，
/// 它将无限执行 block 中的代码，
/// 直到遇到 [break] 或者 [return]
fn loop_expression(sentence: &str) {
    let mut chars= sentence.chars();
    loop {
        let c = chars.next();
        match c {
            Some(c) => print!("{}", c.to_uppercase()),
            None    => break
        }
    }
    println!()
}

fn for_expression(sentence: &str) {
    let chars = sentence.chars();
    for char in chars {
        print!("{}", char.to_uppercase())
    }
    println!()
}