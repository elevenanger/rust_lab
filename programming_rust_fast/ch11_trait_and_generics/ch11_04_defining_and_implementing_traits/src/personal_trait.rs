use crate::Person;

/// Rust 允许为任意的类型实现任意的 trait ，
/// 只要 trait 或者类型在当前的 crate 即可，
/// 通过这种特性，就可以为任意的类型添加任意的方法，
/// 和其它的 trait 一样 ，
/// Young trait 只有当它在作用域时才是可见的，
/// 这种 trait 的目的是为已有的类型添加方法，
/// 称之为 扩展 trait , 属于用户自定义的 trait 。
/// 当你实现一个 trait ，
/// trait 或者类型至少要有一个在当前 crate 是新建的，
/// 这种原则称之为 孤立原则。
trait Young {
    fn is_young(&self) -> bool;
}

impl Young for Person {
    fn is_young(&self) -> bool {
        self.age >= 18 && self.age <= 35
    }
}

#[test]
fn test_young_person() {
    let an = Person {name: "An".to_string(), age: 18};
    let ma = Person {name: "Ma".to_string(), age: 50};

    assert!(an.is_young());
    assert!(!ma.is_young());
}