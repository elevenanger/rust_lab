use std::ops::{Deref, DerefMut};

/// Deref 和 DerefMut trait
/// 使用 Deref 和 DerefMut trait 指定解引用运算符 * 和 . 的行为，
/// 因为 Deref 接收一个 &Self 引用作为参数，并返回一个 &Self::Target 引用，
/// Rust 使用这个方法将前者的引用类型转换为后者的引用类型，
/// 换句话说就是，如果插入一个 deref 调用可以防止类型不匹配，
/// Rust 会自动插入一个。
///
/// 实现 DerefMut 允许对可变引用进行相应的转换，
/// 这被称之为解引用强制转换，
/// 一种类型强制被转换成另一种类型。
///
/// Deref 和 DerefMut 的设计初衷是为了实现智能指针类型，
/// 用于拥有经常通过引用使用的类型，
/// 不要仅仅为了通过强制转换使用 Target 类型方法而实现 Mut 和 DerefMut
struct Selector<T> {
    elements: Vec<T>,
    current: usize
}

impl<T> Deref for Selector<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.elements[self.current]
    }
}

impl<T> DerefMut for Selector<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.elements[self.current]
    }
}

#[test]
fn test_selector() {
    let mut s: Selector<i32> = Selector {
        elements: vec![1, -2, 3, 4],
        current: 1
    };

    // 因为 'Selector' 实现了 'Deref' ，可以使用 '*' 运算符关联当前元素
    assert_eq!(*s, -2);
    // s.abs() 调用了 i32 的方法,
    // 通过强制转换机制,
    // 可以直接使用 'Selector' 调用 i32 的方法
    assert_eq!(s.abs(), 2);
    // 通过赋值 'Selector' 的引用将 -2 的值改变为 5
    *s = 5;
    assert_eq!(s.elements, vec![1, 5, 3, 4]);
}