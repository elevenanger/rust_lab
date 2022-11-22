use std::borrow::{Borrow, BorrowMut};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use crate::Person;

impl PartialEq for Person {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }

    fn ne(&self, other: &Self) -> bool {
        self.name != other.name
    }
}

impl Eq for Person { }

impl Hash for Person {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for c in self.name.as_bytes() {
            c.to_ascii_lowercase().hash(state)
        }
    }
}

#[test]
fn test_person_borrow() {
    let mut p = Person {name: "P".to_string()};
    let q = Person {name: "Q".to_string()};

    assert_eq!(p.borrow().name, "P".to_string());
    p.borrow_mut().name = "PP".to_string();
    assert_eq!(p.borrow().name, "PP".to_string());

    let mut h = HashMap::new();
    h.insert(p.as_ref(), "a".to_string());
    h.insert(q.as_ref(), "q".to_string());

    assert_eq!(h.get(&q).unwrap(),&"q".to_string());
}