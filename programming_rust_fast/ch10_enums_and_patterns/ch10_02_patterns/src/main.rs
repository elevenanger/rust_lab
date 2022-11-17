mod literals_variables_wildcards;
mod tuple_and_struct;
mod array_and_slice;
mod reference;
mod populate_binary_tree;

use ch10_01_enums::enums_with_data::enums_with_data::{RoughTime, TimeUnit};

fn main() {}

/// 使用模式匹配提取枚举中的值：
/// ['RoughTime'] 中的不同的变体的类型不一样，
/// 如果需要取出各个变体中的值，需要使用 match 表达式，
/// match 表达式进行模式匹配，
/// => 符号之前的代码为模式，
/// 模式用来匹配不同的 RoughTime 值，
/// 就和是哟个表达式创建 RoughTime 的值一样,
/// 与之相对的是，
/// 表达式生产值
/// 模式消费值。
///
/// ```
/// match rt {
///    RoughTime::Past(units, count) =>
///        format!("{} {} ago", count, units.plural()),
///    RoughTime::Now => "just now".to_string(),
///    RoughTime::Future(units, count) =>
///    format!("{} {} later", count, units.plural())
/// ```
/// 对于枚举、结构或者元组，
/// 模式匹配对于每个模式从左至右进行扫描，
/// 检查模式中的组件是否能够与值进行匹配，
/// 如果匹配不上，Rust 继续检查下一个模式。
/// ```
/// RoughTime::Future(units, count)
/// ```
/// 当一个模式包括一些简单的标识符，
/// 比如 units count ,
/// 这些会成为模式 => 符号后面代码中的局部变量，
/// ```
/// /* 仅匹配值 count 为 1 的情况
///     这个模式必须在前面，
///     不然就会先匹配 RoughTime::Past(units, count)
///      导致这个模式称为一个无法匹配的模式 */
/// RoughTime::Past(units, 1) =>
///     format!("1 {} past", units.plural()),
/// RoughTime::Past(units, count) =>
///     format!("{} {}s ago", count, units.plural())
/// ```
fn rough_time_desc(rt: &RoughTime) -> String {
    match rt {
        RoughTime::Past(units, 1) =>
            format!("1 {} past", units.plural()),
        RoughTime::Past(units, count) =>
            format!("{} {}s ago", count, units.plural()),
        RoughTime::Now => "just now".to_string(),
        RoughTime::Future(units, 1) =>
            format!("1 {} later", units.plural()),
        RoughTime::Future(units, count) =>
            format!("{} {}s later", count, units.plural())
    }
}

#[test]
fn test_rough_time_desc() {
    let one_years_later = RoughTime::Future(TimeUnit::Year, 1);
    println!("{}", rough_time_desc(&one_years_later));

    let three_years_ago = RoughTime::Past(TimeUnit::Year, 3);
    println!("{}", rough_time_desc(&three_years_ago));
}

#[derive(Debug)]
pub enum Sex {
    Male,
    Female,
}

#[derive(Debug)]
pub struct Person {
    name: String,
    sex: Sex,
    age: u8
}
