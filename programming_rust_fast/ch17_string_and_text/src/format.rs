use std::fmt::{Display, Formatter};

struct Person {
    name: String,
    age: u32
}

/// 为自定义的类型实现 Display trait，
/// 可以通过 {} 打印信息
impl Display for Person {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Person {{name => {}, age => {}}}", self.name, self.age)
    }
}

#[test]
fn test_display_person() {
    let an = Person {name: "an".to_string(), age: 18};
    println!("{}", an);
}
