use ch11_04_defining_and_implementing_traits::{Person, Profiling, CreatureSet};

/// an.grow_up(2) 方法调用，
/// 其实在这个方法调用中有四个角色参与其中：
/// - trait (CreatureSet)
/// - trait 的方法 (grow_up)
/// - 方法的实现 (impl CreatureSet for Person)
/// - 应用于方法实现的实参(&mut an ,2)
/// 我们不需要将这里面所有的角色都在代码中拼出来，
/// Rust 会自动推断出需要调用的 trait 的方法的实现。
///
/// 首先我们需要知道的是方法只是一种特殊的函数,
/// 这两种形式的调用实际上是等价的
/// ```
/// an.grow_up(2);
/// ch11_04_defining_and_implementing_traits::CreatureSet::grow_up(&mut an, 3);
/// ```
///
/// 某些场景下，需要将方法的全限定名在代码中拼出来:
/// - 两个方法具有相同的名字:
/// 在这里 [Greeting] trait 和 [CreatureSet] 都有一个 info 方法，
/// 需要显示声明调用哪个 trait 的方法
/// ```
/// println!("Welcome, {}", Greeting::info(self))
/// ```
/// - 无法推断出 self 实参类型的场景
/// - 使用函数本身作为一个函数值传递的场景
/// - 宏中调用 trait 方法
trait Greeting {
    fn info(&self) -> String;

    fn greeting(&self) {
        println!("Welcome, {}", Greeting::info(self));
    }
}


impl Greeting for Person {
    fn info(&self) -> String {
        format!("{}", Profiling::info(self))
    }
}

#[test]
fn test_greeting() {
    // 全限定名方法调用
    let mut an = <Person as CreatureSet>::from_name_and_age("An", 18);
    // . 运算符方法调用
    an.grow_up(2);
    // 限定名方法调用
    CreatureSet::grow_up(&mut an, 3);
    Greeting::greeting(&an);
}