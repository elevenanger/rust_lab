/// 实际上 ，for 循环只是调用 IntoIterator 和 Iterator 方法的简写,
/// for 循环使用 IntoIterator::into_iter 方法将 &v 转换成一个 iterator,
/// 然后重复调用 Iterator::next 方法，
/// 每次返回 Some(ele) ，
/// for 循环处理 ele，
/// 如果返回 None 则循环结束
///
/// Iterator 的基本术语：
/// - iterator 是任意实现了 Iterator 的类型，
/// - iterable 是实现了 IntoIterator 的类型，使用它的 into_iter 方法可以得到一个 iterator
/// - iterator 产生值
/// - iterator 产生的值是它的元素
/// - 接收 iterator 产生的值的代码称之为消费者(consumer)
///
/// 所有的 Iterator 都实现了 IntoIterator ，
/// 它的 into_iter 方法返回了 iterator 本身
#[test]
fn test_iterator() {
    let v = vec![1, 2, 3, 4];
    for ele in &v {
        print!("{} ", ele);
    }
    println!();

    let mut iterator = (&v).into_iter();
    while let Some(ele) = iterator.next() {
        print!("{} ", ele);
    }
}