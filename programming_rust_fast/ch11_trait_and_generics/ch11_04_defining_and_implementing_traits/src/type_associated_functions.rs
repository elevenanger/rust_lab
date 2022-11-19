use crate::{Person, CreatureSet};

impl CreatureSet for Person {
    fn from_name_and_age(name: &str, age: u8) -> Self {
        Person {name: name.to_string(), age}
    }

    fn grow_up(&mut self, age: u8) {
        self.age += age;
    }
}

/// 参数是 trait 对象，
/// 只能使用 trait 中接收 self 作为参数的函数，
/// 不能使用类型关联函数。
fn creature_grow_up_one_year_older(creature: &mut dyn CreatureSet) {
    creature.grow_up(1);
}

#[test]
fn test_person_from_name_and_age() {
    let an = Person::from_name_and_age("An", 18);
    assert_eq!(an.age, 18);
    assert_eq!(an.name, "An".to_string());
}

#[test]
fn test_creature_grow_up_one_year_older() {
    let mut an = Person::from_name_and_age("An", 18);
    creature_grow_up_one_year_older(&mut an);
    assert_eq!(an.age, 19);
}