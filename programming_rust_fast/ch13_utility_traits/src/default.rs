use crate::Person;

/// Default trait:
/// 用于创建类型的默认值
impl Default for Person {
    fn default() -> Self {
        Person {name: "Anonymous".to_string()}
    }
}

#[test]
fn test_default_person() {
    let a = Person::default();
    assert_eq!(a.name, "Anonymous");
}