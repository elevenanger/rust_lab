#[test]
fn test_trait_object() {
    use std::io::Write;
    let mut buf: Vec<u8> = vec![];
    /*
    let writer: Write = &mut buf;
    不能这样直接声明一个 trait 类型值，
    因为变量的大小必须在编译的时候确定，
    但是实现 Writer trait 的类型可以是任意大小，
    在 java 中声明接口类型对象，是对对象的引用，
    在 Rust 中也一样，
    但是 Rust 需要显式地声明引用类型，
    let writer: &mut dyn Write = &mut buf;
    对于 trait 类型的引用，称为 trait 对象，
    与其他的引用一样，
    trait 对象指向某个值，
    它有生命周期，可以是 mut 或者共享引用。

    Rust 一般在编译时期不知道引用值的类型，
    所以每个 trait 对象都有一部分包括引用类型的额外信息，
    当调用 writer.write_all() 方法时，
    Rust 需要类型信息动态的调用对应的方法，
    Rust 不能直接查询类型信息，
    而且 Rust 不支持从 trait 对象向下转换，
    将 &mut dyn Writer 转换回具体的类型，比如 Vec<u8>,

    在内存中，trait 对象是一个胖指针，
    包括一个指向值的指针和一个指向表示引用值类型表的指针，
    这个表称为 虚拟表(virtual table) 或者 vtable，
    vtable 在编译期间生成，
    所有相同类型的对象共享同一张 vtable ，
    当调用 trait 对象的方法时，
    语言自动使用 vtable 来决定应该调用哪个具体的实现。


    */
    let writer: &mut dyn Write = &mut buf;
    writer.write_all(&[1, 2, 3]).expect("写入失败");
    writer.flush().expect("写入失败");
    assert_eq!(buf, [1, 2, 3]);

    /*
    Rust 会在必要时自动将原始的引用转换成 trait 对象，
    let mut w: Box<dyn Write> = Box::new(&mut buf);
    这里将 &mut buf 自动转换成了 trait 对象，
    这种转换是创建 trait 对象的唯一形式，
    在转换发生的地方， Rust 才知道被引用的类型（这里是 Vec<u8>）,
    在这几添加正确的 vtable 指针，将常规的指针转换为胖指针。
     */
    let mut w: Box<dyn Write> = Box::new(&mut buf);
    w.write_all(&[4, 5, 6]).expect("写入失败");
    w.flush().expect("写入失败");

}