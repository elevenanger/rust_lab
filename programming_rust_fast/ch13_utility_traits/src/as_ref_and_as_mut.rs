use crate::Person;

/// AsRef 和 AsMut trait，
/// 实现这两个 trait 意味着可以很高效的借用 &T
impl AsRef<Person> for Person {
    fn as_ref(&self) -> &Person {
        self
    }
}

impl AsMut<Person> for Person {
    fn as_mut(&mut self) -> &mut Person {
        self
    }
}

#[test]
fn test_person_ref_and_mut_ref() {
    let mut a = Person {name: "an".to_string()};
    assert_eq!(a.as_ref().name, "an".to_string());
    a.as_mut().name = "An".to_string();
    assert_eq!(a.as_ref().name, "An".to_string());
}