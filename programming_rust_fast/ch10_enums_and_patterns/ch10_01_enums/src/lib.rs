pub mod enums_with_data;
pub mod rich_data_structure_using_enums;
pub mod generic_enums;

use std::cmp::Ordering::{self, *};
use std::mem::size_of;

/// Rust 中的枚举类型：
/// enum 关键字，
/// 声明了一个 Sex 枚举类型，
/// 这个枚举类型有三个可能值，
/// 称为构造器或者变体，
/// Sex::male Sex::Female
enum Sex {
    Male,
    Female,
}

/// 和结构一样，
/// 枚举类型也能有自己的方法
impl Sex {
    fn string_form(&self) -> &'static str {
        match self {
            Sex::Male => "male",
            Sex::Female => "female"
        }
    }
}

/// Rust 类 C 风格的枚举在内存中以整数形式存储的，
/// 有些情况下为枚举值指定值是非常有用的，
/// 比如下面这个例子 HttpCode ,
/// 不指定值的情况下 Rust 会为每个枚举值分配从 0 开始的数值，
/// 默认情况下，Rust 使用最小可以适应 C 风格类型枚举值的内置整数类型存储枚举值，
/// 在 HttpCode 中，因为 404 已经超出 u8 类型整数范围，
/// 所以使用 u16 进行存储，每个 HttpCode 在内存中占两个字节。
/// 可以将类 C 风格枚举转换成整数:
/// ```
/// assert_eq!(HttpStatus::Ok as i32, 200);
/// ```
/// 但是不能反过来将整数转换成枚举值，
/// 因为 Rust 需要确保枚举值只能是在枚举定义中声明的值，
/// 未经检查的转换可能会破坏这一保证,
/// 可以自己定义一个检查转换函数：
/// ```
/// fn http_status_from_u32(n: u32) -> Option<HttpStatus> {
/// match n {
///     200 => Some(HttpStatus::Ok),
///     404 => Some(HttpStatus::NotFound),
///     _ => None
/// }
/// ```
/// 和结构一样，加上注解编译器可以自动为枚举类型实现支持 == 运算符等特性，
/// ```
/// #[derive(Copy, Clone, Debug, PartialEq, Eq)]
/// ```
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum HttpStatus{
    Ok = 200,
    NotFound = 404,
}

fn http_status_from_u32(n: u32) -> Option<HttpStatus> {
    match n {
        200 => Some(HttpStatus::Ok),
        404 => Some(HttpStatus::NotFound),
        _ => None
    }
}

/// 比较两个 i32 类型数值，
/// 这里使用到了 Ordering 枚举类型 ，
/// ```
/// use std::cmp::Ordering::{self, *};
/// ```
/// 引入 Ordering 枚举中所有的构造器。
fn compare(n: i32, m:i32) -> Ordering {
    if n < m{
        Less
    } else if n > m {
        Greater
    } else {
        Equal
    }
}

#[test]
fn test_compare() {
    let result = compare(10, 20);
    assert_eq!(result, Less);
}

/// 枚举类型的值：
#[test]
fn test_enum_value() {
    assert_eq!(Sex::Male as u8, 0);
    assert_eq!(Sex::Female as u8, 1);
    assert_eq!(size_of::<Sex>(), 1);

    assert_eq!(HttpStatus::Ok as i32, 200);
    assert_eq!(HttpStatus::NotFound as i32, 404);
    assert_eq!(size_of::<HttpStatus>(), 2);
}

#[test]
fn test_http_status_from_u32() {
    let status = http_status_from_u32(200);
    assert_eq!(status, Some(HttpStatus::Ok));
}

#[test]
fn test_sex_string_form() {
    let male = Sex::Male;
    let female = Sex::Female;

    assert_eq!(male.string_form(), "male");
    assert_eq!(female.string_form(), "female");
}