use crate::Person;
use crate::self_in_traits::Change;

/// 子 trait :
/// 将一个 trait 定义为另一个 trait 的扩展。
/// ChangeName: Change
/// 所有的 ChangeName 类型都是 Change 类型，
/// 实现 ChangeName 的类型都需要实现 Change
/// subtrait : ChangeName
/// supertrait : Change
///
/// 需要实现 subtrait 的类型
/// subtrait 和 supertrait 都要在其作用域内。
trait ChangeName: Change {
    fn change_name(&mut self, new_name: &str);
}

impl ChangeName for Person {
    fn change_name(&mut self, new_name: &str) {
        self.name = new_name.to_string();
    }
}

#[test]
fn test_change_name_for_person() {
    let mut an = Person {name: "An".to_string(), age: 18};
    an.change_name("Ann");

    assert_eq!(an.name, "Ann");
}