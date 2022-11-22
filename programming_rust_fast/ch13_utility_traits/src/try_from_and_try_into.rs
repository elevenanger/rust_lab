use crate::Person;

/// TryFrom trait
/// 实现了 TryFrom 则自动实现了 TryInto
impl TryFrom<String> for Person {
    type Error = std::fmt::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.len() {
            0..=10 => Ok (Person {name: value}),
            _ => Err(std::fmt::Error)
        }
    }
}

#[test]
fn test_try_from() {
    let an = Person::try_from("An".to_string())
                        .expect("转换错误");
    let name = <Person as TryInto<String>>::try_into(an)
                        .expect("转换错误");
    assert_eq!(name, "An".to_string());
}