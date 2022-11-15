fn main() {}

struct Person {
    name: String,
    age: u32
}

impl Person {
    /// impl 块中的函数不一定需要 self 作为参数，
    /// 这些函数仍然是类型关联函数，
    /// 因为它们在 impl 代码块中，
    /// 但是它们不是方法，
    /// 因为没有 self 作为参数，
    /// 为了和方法进行区分，
    /// 称之为 类型关联函数
    pub fn with_name_and_age(name: &str, age: u32) -> Person {
        Person {name: name.to_string(), age}
    }
}

#[test]
fn test_person() {
    let an = Person::with_name_and_age("An", 19);

    assert_eq!(&an.name, "An");
    assert_eq!(an.age, 19);
}