/// Copy 类型，
/// 在多数场景下，
/// 赋值会 move 值，
/// 但是不用有任何资源的简单类型，可以是 Copy 类型，
/// Copy 类型赋值会进行拷贝源值，而不是 move 源值，
/// Copy 是一个标记 trait ，
/// Rust 允许只需要进行逐个字节浅拷贝的类型实现 Copy trait，
/// 拥有其它资源的类型，比如 堆缓冲区 不能实现 Copy trait ，
/// 任何实现 Drop trait 的类型不能是 Copy 类型，
/// Rust 推断一个类型如果需要特殊的清理代码，
/// 那么就同样需要特殊的 Copy 代码，
/// 所以它不能实现 Copy trait ，
/// 可以通过 Rust 派生 Copy 和 Clone trait ，
/// 不需要自己实现。
#[derive(Debug)]
struct MySimpleType {
    i: i32,
    j: i32,
}

impl Clone for MySimpleType {
    fn clone(&self) -> Self {
        MySimpleType {
            i: self.i,
            j: self.j
        }
    }
}

impl Copy for MySimpleType {}

impl PartialEq for MySimpleType {
    fn eq(&self, other: &Self) -> bool {
        self.i == other.i && self.j == other.j
    }

    fn ne(&self, other: &Self) -> bool {
        self.i != other.i || self.j != other.j
    }
}

#[test]
fn test_my_type() {
    let a = MySimpleType {i: 1, j: 1};
    let b = a;

    assert_eq!(a, b);
}