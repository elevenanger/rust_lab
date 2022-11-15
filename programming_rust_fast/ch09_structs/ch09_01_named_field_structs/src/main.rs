fn main() {
}

/// 命名字段结构：
/// ```
/// struct Person {
///     name: String,
///     age: u8,
/// }
/// ```
/// 声明一个名为 [`Person`] 的结构，
/// 类型名使用驼峰命名，
/// 结构内声明每个字段的字段名和字段类型。
/// 结构和结构中的字段默认是 private 访问权限，
/// 只对声明结构的模块和子模块可见，
/// 可以在结构和字段的定义加上 pub 关键字使得其对于其它模块可见，
/// 结构定义为 pub ，字段不定义为 pub 默认还是 private 的访问权限。
/// 其它使用该结构的模块可以使用这个结构的 public 函数，
/// 但是无法访问 private 字段，或者使用结构表达式创结构的值，
/// 创建结构的值需要所有的结构字段都是可见的，
/// 所以不能通过结构表达式创建 String 类型值，
/// 只能通过 public 类型相关函数，比如 Vec::new().
pub struct Person {
    name: String,
    age: u8,
}

/// 构造一个 [`Person`]
/// ```
/// Person {
///     name: name.to_string(),
///     age
/// }
/// ```
/// 结构表达式从类型名开始，
/// 然后列出字段名以及各个字段对应的值。
/// 可以使用相同名字和类型的局部变量或者参数填充字段值.
fn new_person(name: &str, age: u8) -> Person {
    Person {
        name: name.to_string(),
        age
    }
}

/// 创建一个命名字段的结构，可以使用其它相同结构的值提供缺省字段的值，
/// 在结构表达式中，如果字段名后面跟着 .. 表达式，
/// 则任何未提及的字段的值都来自于该表达式，
/// 这个表达式必须是另外一个相同类型结构的值。
fn split_people(p: Person) -> (Person, Person) {
    let mut p1 = Person {.. p};
    /* 因为 String 不是 Copy 类型，必须显式地克隆它 */
    let mut p2 = Person {name: p1.name.clone(), .. p1};
    p1.name.push_str(" I");
    p2.name.push_str(" II");
    (p1, p2)
}

#[test]
fn test_create_person() {
    let an = new_person("An", 10);
    /* 使用 . 运算符获取结构中的字段 */
    assert_eq!(an.name, "An");
    assert_eq!(an.age, 10);
}

#[test]
fn test_split_people() {
    let an = new_person("An", 10);
    let (an_1, an_2) = split_people(an);
    assert_eq!(an_1.name, "An I");
    assert_eq!(an_1.age, 10);
    assert_eq!(an_2.name, "An II");
    assert_eq!(an_2.age, 10);
}