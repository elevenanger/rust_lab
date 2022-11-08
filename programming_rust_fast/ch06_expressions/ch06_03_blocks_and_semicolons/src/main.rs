fn main() {
    produce_value();

    declarations();

    nesting_functions();
}

struct Person {name: String, address: String}

/// 代码块产生值：
/// 代码块是最常见的表达式，
/// 可以产生值并且适用于任何地方。
/// ```
/// let desc = match wong.address.contains("Fu") {
/// true => {format!("{} lives in {}.", wong.name, wong.address)}
/// false => {format!("{} is a mystery man.", wong.name)};
/// ```
/// - 花括号括起来的表达式没有 ; 结尾，在 rust 中，这个表达式的值将视为整个 {} 表达式的值;
/// - 声明必须要以 ; 结尾；
/// - 以 ; 结尾的方法调用，但是没有进行赋值的话，返回值将会被弃用。
fn produce_value() {

    let wong = Person {name: "Wong".to_string(),
                        address: "Fu road".to_string()};

    let desc = match wong.address.contains("Fu") {
        true => { format!("{} lives in {}.", wong.name, wong.address) }
        false => { format!("{} is a mystery man.", wong.name) }
    };
    println!("{}", desc);
}

/// 声明：
/// 除了表达式和分号，代码块中可以出现任意数量的声明。
/// ```
/// let name: type = expr;
/// ```
/// 最常见的 [let] 声明，声明一个局部变量，
/// [let] 声明的变量可以不进行初始化，
/// 变量可以在后续进行赋值。
/// ```
/// let age = 10;
/// let is_adult;
/// if age > 18 {
/// is_adult = true;
/// } else {
/// is_adult = false;
/// }
/// assert!(!is_adult);
/// ```
fn declarations() {
    let age = 10;
    let is_adult;
    if age > 18 {
        is_adult = true;
    } else {
        is_adult = false;
    }
    assert!(!is_adult);
}

/// 代码块中定义函数：
/// 在函数中可以定义函数，
/// 当 [fn] 在代码块中声明，
/// 它的作用域是整个代码块，可以在整个代码块中使用，
/// 但是无法访问代码块中函数外的局部变量或者参数。
fn nesting_functions() {
    let x = 10;
    fn nested_functions(num: i32) -> i32 {
        let y = num * 10;
        y
    }
    println!("{}", nested_functions(x));
}