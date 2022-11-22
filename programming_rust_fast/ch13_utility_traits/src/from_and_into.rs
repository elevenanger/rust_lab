use crate::Person;

/// From trait
impl From<&str> for Person {
    fn from(name: &str) -> Self {
        Person {
            name: name.to_string()
        }
    }
}

/// Into trait
impl Into<String> for Person {
    fn into(self) -> String {
        self.name
    }
}

#[test]
fn test_person_from() {
    let p = Person::from("An");
    assert_eq!(p.name, "An".to_string());
    assert_eq!(<Person as Into<String>>::into(p), "An".to_string());
}