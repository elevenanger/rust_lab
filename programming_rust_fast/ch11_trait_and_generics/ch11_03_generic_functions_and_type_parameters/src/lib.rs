use std::fmt::Debug;
use std::fs::File;
use std::hash::Hash;
use std::io::Write;

/// say_hello 函数重写为泛型函数：
/// W: Write 是类型参数，
/// 表示在整个函数体中，
/// W 表示实现了 Write trait 的类型，
fn say_hello<W: Write>(out: &mut W) -> std::io::Result<()> {
    out.write_all(b"hello you\n").expect("写入失败");
    out.flush()
}

#[test]
fn test_say_hello() {
    let mut local_file = File::create("hello.txt").expect("文件创建失败");
    /*
    当你将 &mut local_file 作为参数传递给 say_hello() 泛型函数，
    实际调用的是 say_hello() 里面实际调用的函数是 File::write() 和 File::flush(),
    当你将 &mut a 作为参数传递给泛型函数 say_hello()，
    实际调用的是 say_hello::<Vec<u8>>() ,
    Rust 为了这些不同版本的方法分别生成机器字，
    调用对应的方法，
    在这两个场景中，
    Rust 从实参中推断 W 的类型，
    这个过程称为单态化，
    由编译器自动处理。
     */
    say_hello(&mut local_file).expect("文件写入失败");

    let mut a = vec![];
    say_hello(&mut a).expect("数组写入失败");
    assert_eq!(a, b"hello you\n");

    /*
    可以显式地拼出完整的类型，
    不过一般不需要这么做
    Rust 会从实参中个推断出类型,
    只有在当前上下文中，实参存在多种可能性，无法提供有用的信息时，
    才需要显式地拼出类型参数。
     */
    say_hello::<Vec<u8>>(&mut a).expect("数组写入失败");

}

/// 具备多个能力的类型参数:
/// 类型参数 <T: Debug + Hash + Eq + Ord> ，
/// 表示同时实现了 Debug Hash Eq Ord trait 的类型.
fn max<T: Debug + Hash + Eq + Ord>(values: &mut Vec<T>) {
    values.sort();
    values.reverse();
    println!("{:?}", values.get(0));
}

#[test]
fn test_top_ten() {
    let mut a = (0..1000).collect();
    max(&mut a);
}

/// 泛型函数可以由多个类型参数：
/// <A: Debug, B: Debug>
fn print_info<A: Debug, B: Debug>(a: A, b: B) {
    println!("{:?} : {:?}", a, b);
}

#[test]
fn test_print_info() {
    print_info("score", 100);
}

/// 类型参数的另一种写法：
fn print_info_ver2<A, B>(a: A, b: B)
    where A: Debug,
          B: Debug {
    println!("{:?} : {:?}", a, b);
}

#[test]
fn test_print_info_v2() {
    print_info_ver2("score", 100);
}

/// 泛型函数也可以有生命周期参数，
/// 生命周期参数在类型参数之前：
/// <'a, 'b, A, B>
fn print_info_ver3<'a, 'b, A, B>(a: &'a A, b: &'b B)
    where A: Debug,
          B: Debug {
    println!("{:?} : {:?}", a, b);
}

#[test]
fn test_print_info_v3() {
    print_info_ver3(&"score", &100);
}
