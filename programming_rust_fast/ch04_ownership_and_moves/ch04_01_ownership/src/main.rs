fn main() {
    variable_lifetime();

    box_lifetime();

    composer_lifetime();
}

/// 变量的生命周期
/// Rust 中所有权的概念内置在语言中并且在编译时执行强制检查
/// 每个值都有唯一的所有者决定它的生命周期
/// 当所有者被释放（废弃）
/// 所拥有的值也会被废弃
fn variable_lifetime() {
    /*
     变量对它的值具有所有权
     当控制离开变量所声明的块
     变量会被废弃
     相应的它所拥有的值也会被废弃
     nums 在这个语句中进行内存的分配
     */
    let mut nums = vec![1, 2, 3, 4, 5];
    for num in 6..11 {
        nums.push(num);
    }
    println!("{:?}", nums);
    // nums 变量的作用域在这里结束，变量和它所拥有的值在这里被废弃
}

/// box 类型变量的生命周期
/// Box 类型是所有权的另一个例子
/// Box<T> 是一个指向存储在对内存上 T 类型值的指针
/// Box::new(v) 分配堆内存，将 v 的值移动到其中
/// 并且返回一个 Box 指针，指向堆内存上的空间
/// 一旦 Box 拥有它指向的内存空间
/// 当 Box 被废弃的同时，堆内存上的空间也会被释放
fn box_lifetime() {
    let score = Box::new((12, 11)); /* 在这里分配内存空间 */
    let label = format!("{:?}", score); /* 在这里分配内存空间 */
    println!("{:?} {}", score, label); /* 两个变量都在这里废弃 */
}

/// 和变量拥有值一样
/// 结构也对它的域具有所有权
/// 元组、数组、矢量对于它的元素也有所有权
///
/// 在下面这个例子中，组合者是 students
/// 一个 Person 结构的 vector
/// 这里面存在比较多的所有权关系
/// 但是每层关系都是比较直接的
/// students 有 vector 的所有权
/// vector 有其中每个元素（Person）的所有权
/// 每个 Person 结构有其中所有域的所有权
/// 每个 name(String) 域对于它的文本具有所有权
/// 每个 age(i32) 域对于它的数值有所有权
/// 当程序控制离开声明 students 的作用域
/// 程序会丢弃它的值,并控制整个过程
///
/// 在 rust 中，所有者和它所拥有的值是一个树状形式
/// 你的所有者是你的父节点
/// 你拥有的值是你的子节点
/// 每棵树最终的根节点是一个变量
/// 当变量的离开了它的作用域范围
/// 整棵树都会随之丢弃
/// 这棵树是由多种类型混合而成的
/// Rust 的单一所有者原则禁止任何重新聚合的结构
/// 那会使得布置会比树更加复杂
/// Rust 中的每个值都属于一棵树，根节点是某个变量
///
/// Rust 不需要显示丢弃值
/// Rust 丢弃值的方式是在变量离开它的作用域或者从 vector 中删除被删除的元素等
/// 从所属权的树中移除它
/// 在这个时候 Rust 确保值被妥善丢弃
/// 连带它所拥有的任何其它的东西
///
/// Rust 对于所有权属性进行了一些扩展
/// 1、可以将值从一个所有者转换至另一个所有者，这个特性使你可以构造、重新安排、或者摧毁树
/// 2、非常简单的类型（整数、浮点数、字符）从所有者关系中排除，将这些类型称为 复制 类型
/// 3、标准库提供引用计数类型 RC 和 Arc 它允许值在某些约束下有多个所有者
/// 4、可以为一个值借用引用，引用是没有归属的指针，拥有有限的生命周期
///
fn composer_lifetime() {
    struct Person { name: String, age: i32, }
    let mut students = Vec::new();
    students.push(Person {name: "Ming".to_string(), age: 12});
    students.push(Person {name: "Gang".to_string(), age: 13});
    for student in students {
        println!("name => {} , age => {}", student.name, student.age);
    }
}