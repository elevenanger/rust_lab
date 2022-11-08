fn main() {
    if_expressions();

    match_expressions();
}

/// if 表达式：
/// - 每个 [condition] 表达式都必须为 [bool] 类型值
/// ```
/// if condition1 {
///     block1
/// } else if condition 2 {
///     block2
/// } else {
///     block3
/// }
/// ```
///
/// - 使用 [if] 表达式进行赋值，每个分支的返回值都必须是相同类型:
/// ```
/// let grade = if num > 0 && num < 60 {
///     'F'
/// } else if num >= 60 && num < 70  {
///     'D'
/// } else if num >= 70 && num < 80 {
///     'C'
/// } else if num >= 80 && num < 90 {
///     'B'
/// } else if num >= 90 && num <= 100 {
///     'A'
/// } else {
///     'G'
/// };
/// ```
fn if_expressions() {
    if_condition(10);

    let_if(0);
}

fn if_condition(num: i32) {
    if num > 100 {
        println!("error score.");
    } else if num < 0 {
        println!("score = {}", num);
    } else {
        println!("what?");
    }
}

fn let_if(num: u8) {
    let grade = if num > 0 && num < 60 {
        'F'
    } else if num >= 60 && num < 70  {
        'D'
    } else if num >= 70 && num < 80 {
        'C'
    } else if num >= 80 && num < 90 {
        'B'
    } else if num >= 90 && num <= 100 {
        'A'
    } else {
        'G'
    };
    println!("Grade => {}", grade);
}

/// match 表达式：
/// [match] 表达式类似于 C 语言中的 [switch] 语句，但是更加灵活，
/// [match] 表达式的多样性取决于 => 左值支持的 [模式] 的多样性，
/// [模式] 可以匹配一个范围的值，[Rust] 中的模式是它特有的迷你语言。
///
/// - [Rust] 匹配机制：
/// [Rust] 从第一个开始检查每个 [pattern] 的值，
/// 如果匹配上了，对应的 [expr] - 表达式 会进行求值，
/// [match] 表达式结束。
/// 需要确保至少要有一个 [pattern] 能够匹配上 [value]，
/// [Rust] 禁止 [match] 表达式没有囊括所有的可能值。
///
/// - [match] 表达式通用的表示形式如下：
/// ```
/// match value {
///     pattern => expr,
///     ...
/// }
/// ```
/// 如果是语句块，expr 后面的 , 可以省略。
///
/// - [match] 表达式应用于常量:
/// ```
/// match code {
/// 0 => println!("Ok"),
/// 1 => println!("Fail"),
/// _ => println!("Unknown"),
/// }
/// ```
/// [_] 通配符表示匹配任意内容，类似于 [switch] 语句中的 [default] ,
/// 它只能放在 [match] 语句中的末尾，它会优先于其它分支，导致后续的分支无法继续执行。
///
/// - [match] 语句应用于 [Option] 类型值：
/// ```
/// match name {
/// Some(name) => println!("Hello, {}", name),
/// None => println!("Greetings, stranger.")
/// }
/// ```
/// - 使用 [match] 表达式进行赋值，每个模式匹配的表达式求值结果必须是相同的类型：
/// ```
/// let grade = match score {
///     1 ..=59  => 'F',
///     60..=69  => 'D',
///     70..=79  => 'C',
///     80..=89  => 'B',
///     90..=100 => 'A',
///     _        => 'G'
/// };
/// ```
fn match_expressions() {
    match_code(1);

    let name: Option<String>;
    name = Some("An".to_string());
    match_options(name);

    let_match(90);
}

fn match_code (code: u8) {
    match code {
        0 => println!("Ok"),
        1 => println!("Fail"),
        _ => println!("Unknown"),
    }
}

fn match_options(name: Option<String>) {
    match name {
        Some(name) => println!("Hello, {}", name),
        None => println!("Greetings, stranger.")
    }
}

fn let_match (score: u8) {
    let grade = match score {
        1 ..=59  => 'F',
        60..=69  => 'D',
        70..=79  => 'C',
        80..=89  => 'B',
        90..=100 => 'A',
        _        => 'G'
    };
    println!("Grade => {}", grade);
}