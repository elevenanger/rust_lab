use crate::Person;

/// trait 可以使用 Self 关键字作为类型使用
/// 表示和实现该 trait 并实际使用 trait 函数实例的相同类型。
pub trait Change {
    // Self 关键字作为返回值类型
    fn change_info(&mut self) -> Self;
    // Self 关键字作为形参类型
    fn exchange(&mut self, another: &mut Self);
}

impl Change for Person {
    fn change_info(&mut self) -> Self {
        let before_change = self.clone();
        let age = self.age;
        self.age = age + 1;
        before_change
    }

    fn exchange(&mut self, another: &mut Self) {
        let another_name = another.name.clone();
        another.name = self.name.clone();
        self.name = another_name;
    }
}

#[test]
fn test_change_person() {
    let mut wang = Person {name: "Wang".to_string(), age: 34};
    let old_wang = wang.change_info();
    assert_eq!(wang.age, 35);
    assert_eq!(old_wang.age, 34);
}

#[test]
fn test_person_exchange() {
    let mut an = Person {name: "An".to_string(), age: 18};
    let mut ma = Person {name: "Ma".to_string(), age: 50};
    an.exchange(&mut ma);

    assert_eq!(an.name, "Ma".to_string());
    assert_eq!(ma.name, "An".to_string());
}