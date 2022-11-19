use std::fs::File;
use std::io::Write;

/// Rust 中提供了两种支持多态的特性 ： trait 和 泛型,
/// Write 参数表示任何实现了 Write trait 的类型，
/// 参数 out 表示任何实现了 write trait 类型的引用,
/// 使用 trait 必须确保在作用域内通过 ues 引入 trait,
/// 一些特殊的 trait 包括 Clone 和 Iterator 不需要引入即可直接使用，
/// 是因为它们默认在所有的作用域内，这是 Rust 的 prelude trait ，
/// 自动引入所有的模块。
///
/// 通过 &mut dyn Write 会产生动态委派的开销，
/// 称为动态方法调用，
/// 该开销由 dyn 关键字指定，
/// dyn writer 被称为 trait 对象。
pub fn say_hello(out: &mut dyn Write) -> std::io::Result<()> {
    out.write_all(b"hello you\n")?;
    out.flush()
}

#[test]
fn test_say_hello() {
    let mut local_file = File::create("hello.txt").expect("文件创建失败");
    say_hello(&mut local_file).expect("数据写入异常");

    let mut bytes = vec![];
    say_hello(&mut bytes).expect("数据写入异常");
    assert_eq!(bytes, b"hello you\n");
}

/// 泛型参数：
/// <T: Ord> 表示任意实现了 Ord trait 的类型，
/// 这种需求称之为约束，因为它限定了 T 的类型范围。
pub fn min<T: Ord> (val1: T, val2: T) -> T {
    if val1 <= val2 {
        val1
    }
    else {
        val2
    }
}

#[test]
fn test_min() {
    assert_eq!(min(10, 20), 10);
    assert_eq!(min('a', 'x'), 'a');
}