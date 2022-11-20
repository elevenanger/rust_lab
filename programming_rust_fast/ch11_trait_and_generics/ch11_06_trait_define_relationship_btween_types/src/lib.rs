mod generic_traits;
mod impl_trait;

struct Person {
    name: String,
    age: u8,
}

impl Person {
    fn new(name: &str, age: u8) -> Person {
        Person {name: name.to_string(), age}
    }
}

impl Iterator for Person {
    /// Item 是关联类型，
    /// 所有实现 Iterator trait 的类型，
    /// 都必须指定关联类型,
    /// 关联类型特别适合每个实现都有一个特定关联类型的场景。
    type Item = Person;

    fn next(&mut self) -> Option<Self::Item> {
        match self.age {
            0..=120 => {
                self.age += 1;
                Some(Person {name: self.name.to_string(), age: self.age})
            }
            _ => None
        }
    }
}

/// trait 兑现可以指定关联类型
fn dump(iter: &mut dyn Iterator<Item = Person>) {
    for _ in 0..10 {
        let per = iter.next().unwrap();
        println!("name : {} , age : {}", per.name, per.age);
    }
}

#[test]
fn test_person_iter() {
    let mut an = Person::new("An", 18);
    dump(&mut an);
}


#[test]
fn test_i32_iterator() {
    let mut a = [1, 2, 3].into_iter();
    assert_eq!(1, Iterator::next(&mut a).unwrap());
}

