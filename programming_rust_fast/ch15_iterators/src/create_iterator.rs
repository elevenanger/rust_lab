use std::fmt::Debug;
use std::iter::{from_fn, successors};
use std::path::{Path};

/// 所有类型都可以实现 iter 和 iter_mut
/// path.iter() 产生的是 &OsStr 类型元素
/// 如果一种类型存在多种遍历方式，
/// 这种类型通常会为每种遍历方式提供特特定的方法。
#[test]
fn test_create_iterator() {
    let path = Path::new("/Users/Projects/rust_lab");
    dump(path);
}

fn dump<T, U> (t: T)
where   T: IntoIterator<Item=U>,
        U: Debug
{
    for u in t {
        println!("{:?}", u);
    }
}

/// 调用 from_fn() 创建一个迭代器
/// 迭代器中的每个元素都会调用作为参数的闭包，
/// 因为迭代器总是返回 Some 类型值，
/// 序列将不会终止，
/// 调用 take(100) 方法限制其只获取前100个元素,
/// collect() 方法从结果迭代中创建 Vec,
/// 这是一种高效构造和初始化 vector 的方式
#[test]
fn test_from_fn() {
    let mut i = 0;

    let mut lengths: Vec<i32> =
        from_fn(|| {
            i += 2;
            Some(i)
        }).take(100).collect();

    assert_eq!(lengths.pop(), Some(200));
    assert_eq!(i, 200);
}

/// successor() 函数，
/// 提供一个初始值，和一个函数，
/// 函数接收初始值作为参数，
/// 返回 Option 值作为下个值，
/// 如果返回 None 则迭代结束。
///
/// from_fn 和 successor 都接收 fn_mut 作为参数.
/// 所以闭包可以从作用域中可以捕获并修改变量作为其运行时.
#[test]
fn test_successor() {
    let a = successors(Some(1), |&x| {Some(x + 1)} )
        .enumerate()
        .find(|(_i, x)| x % 5 == 0)
        .take();

    println!("{:?}", a);
}

fn fibonacci(n: usize) -> Vec<i32> {
    let mut init = (0, 1);

    from_fn(|| {
        init = (init.1, init.0 + init.1);
        Some(init.0) })
        .take(n)
        .collect::<Vec<_>>()
}

#[test]
fn test_fibonacci() {
    let fibs = fibonacci(10);
    println!("{:?}", fibs);
}

/// drain 方法接收一个集合的 mut 引用作为参数，
/// 返回一个 Iterator 将每个元素的所有权转移给消费者，
/// 和 into_iter 不一样的是，
/// into_iter 按值接收 Iterator 并消费它，
/// drain 仅仅借用了集合的 mut ref，
/// 当丢弃迭代器时，
/// 它会将集合剩余的元素移除并将其留空,
/// 如果是可以通过 range 索引的类型，
/// drain 方法接收一个区间的元素进行移除，
/// 而不是移除整个序列。
#[test]
fn test_drain() {
    let mut outer = "mutability".to_string();
    let inner = String::from_iter(outer.drain(0..3));

    assert_eq!(outer, "ability");
    assert_eq!(inner, "mut");
}