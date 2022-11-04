use std::rc::Rc;

fn main() {
    moves();

    more_operation();

    moves_and_control_flow(10);

    moves_and_indexed_content();

    copy_types();

    rc_and_arc();
}

/// Rust 中的 move 特性
/// Rust 对于大多数类型的赋值将值从源变量 move 到目标变量
/// 源变量会变成未初始化的状态
///
/// # Examples:
/// - move 特性
///
/// ```
/// let s = vec!["An".to_string(), "ger".to_string()];
/// let t = s;
/// let u = t;
/// ```
/// - s 初始化，具有 vector 的所有权
/// ```
/// let s = vec!["An".to_string(), "ger".to_string()];
/// ```
/// - 将 vector 的所有权 move 给 t , s 变成未初始化值
/// ```
/// let t = s;
/// ```
/// let u = t; 同理
///
/// 如果需要 s 还需要使用，可以使用 clone() 方法进行深拷贝，将拷贝的值赋值给其它的变量
/// ```
/// let t = s.clone();
/// ```
fn moves() {
    let s = vec!["An".to_string(), "ger".to_string()];
    /* 使用 clone 进行深拷贝 */
    let t = s.clone();
    let u = s.clone();

    println!("{:?}", u);
    println!("{:?}", s);
    println!("{:?}", t);
}

/// 更多的 move 操作
///
/// - 对于一个已赋值的变量再次进行赋值操作，原来的值将会被丢弃
/// ```
/// let mut s = "string1".to_string();
/// s = "string2".to_string();
/// ```
/// - 使用转移将值的所有权转移到另一个变量，两个值都会被保留
/// ```
/// s = "string3".to_string();
/// let s_2 = s;
/// s = "string4".to_string();
/// ```
///
/// 除了赋值和初始化，还可能发生 move 的场景
///
/// - 从函数返回值
/// ```
/// let strings = Vec::new()
///
/// 调用 Vec::new() 构造了一个新的 vector 并返回
/// 返回的不是一个指向 vector 的指针
/// 而是 vector 本身
/// 它的所有权从 Vec::new() 函数 move 给了 strings 变量
/// ```
///
/// - 构造新的值
/// ```
/// let wong = {name: "wong wu".to_string() age: 18}
/// Person 结构的 name 域通过 to_string 函数的返回值完成初始化
/// 这个结构具有这个字符串的所有权
/// ```
///
/// - 向函数传值
/// ```
/// let persons = Vec::new()
/// persons.push(wong)
/// 整个 Person 结构 wong ,而不是其指针，通过 push 方法 move
/// vector 取得了 wong 的所有权
/// 并具备了 name 域的间接所有权
/// ```
///
/// move 移动的是值的属性
/// 而不是它们所拥有的堆内存空间
///
fn more_operation() {
    /* s 赋值为 string1 */
    let mut s = "string1".to_string();
    /* s 赋值为 string2，原来的值 string1 被丢弃 */
    s = "string2".to_string();

    println!("s => {}", s);

    s = "string3".to_string();
    let s_2 = s; /* 将所有权 move 到 s_2 */
    s = "string4".to_string();
    println!("s => {} s_2 = > {}", s, s_2);

}

/// move 与控制流
/// ```
///    let mut nums = vec![10, 20, 30];
///    if i < 2 {
///        f(nums);
///    } else {
///        g(nums);
///    }
///    h(nums);
///
/// f(nums) 是可以的，将 nums move 给 f 函数
/// g(nums) 也是可以的，将 nums move 给控制流的另一个分支
/// h(nums) 是错误的，因为 nums 的值已经 move 到控制流的函数中
/// nums 此时是未初始化的状态
///    let mut nums = vec![10, 20, 30];
///    if i < 2 {
///        f(nums);
///        nums = create_vec_i32();
///    } else {
///        g(nums);
///        nums = create_vec_i32();
///    }
///    h(nums);
/// 在每个控制流的分支中重新对 nums 变量进行赋值即可
/// 确保在传递给 h 函数的时候 nums 是一个有值的状态
/// ```
fn moves_and_control_flow(i: i32) {
    let mut nums = vec![10, 20, 30];
    if i < 2 {
        f(nums);
        nums = create_vec_i32();
    } else {
        g(nums);
        nums = create_vec_i32();
    }
    h(nums);
}

fn create_vec_i32() -> Vec<i32> {
    vec![20, 30, 40]
}

fn f(nums: Vec<i32>) {
    println!("f {:?}", nums);
}

fn g(nums: Vec<i32>) {
    println!("g {:?}", nums);
}

fn h(nums: Vec<i32>) {
    println!("h {:?}", nums);
}

/// move 和索引内容
/// - 使用索引值给变量赋值
/// ```
/// let mut a = vec![1, 2, 3, 4];
/// let a0 = a[0];
/// let a1 = a[1];
///
/// 这种赋值方式在 Rust 中是不允许的
/// 因为赋值会 move 所有权
/// let a0 = a[0];
/// 将 a 对 a[0] 的所有权转移给 a0 变量
///
/// vector 需要携带更多的信息表示哪个元素任然具备所有权
/// 哪个元素已经变成未初始化的状态
/// vector 应该仅仅作为一个 vector 即可
/// ```
///
/// 可以使用以下几种方式从有索引的序列中取值对变量进行赋值
/// - 使用 pop 方法弹出序列中最后的元素
/// ```
/// let mut a = vec![1, 2, 3, 4, 5];
///
/// let last = a.pop();
/// assert_eq!(last, Some(5 as i32));
/// ```
///
/// - swap_remove(i) 给定索引，将序列中对应的元素移除，并且将最后一个元素移动到索引位置
/// ```
/// let first = a.swap_remove(0);
/// assert_eq!(first, 1);
/// ```
///
/// - std::mem::replace 使用另外的值对元素进行替换
/// ```
/// let second = std::mem::replace(&mut a[1], 10);
/// assert_eq!(second, 2);
/// assert_eq!(a[1], 10);
/// ```
/// - 这三种方法都可以将元素从序列中移除，但是任然保持了序列是满的状态
///
/// - 从结构中移出值的方法
/// ```
/// struct Person {name: Option<String>, age: i32};
/// let mut zhang = Person {name: Some("Zhang".to_string()), age: 12};
/// let zhang_name =
/// std::mem::replace(&mut zhang.name, None);
/// println!("zhang_name => {:?}", zhang_name);
/// assert_eq!(zhang.name, None);
///
/// name 类型 Option<String> 说明 None 值也是合法的
/// 使用 None 值替换原值 move 所有权给另一个变量
/// 为了简化这种写法
/// take() 函数和  replace() 可以产生相同的效果
/// ```
///
fn moves_and_indexed_content() {
    let mut a = vec![1, 2, 3, 4, 5];

    let last = a.pop();
    assert_eq!(last, Some(5 as i32));

    let first = a.swap_remove(0);
    assert_eq!(first, 1);

    let second = std::mem::replace(&mut a[1], 10);
    assert_eq!(second, 2);
    assert_eq!(a[1], 10);

    println!("a => {:?}", a);

    struct Person {name: Option<String>, age: i32};

    let mut zhang = Person {name: Some("Zhang".to_string()), age: 12};
    let zhang_name =
        std::mem::replace(&mut zhang.name, None);
    println!("zhang_name => {:?}", zhang_name);
    assert_eq!(zhang.name, None);

    zhang.name = Some("Zhang san".to_string());
    let zhang_san = zhang.name.take();
    println!("zhang_san => {:?}", zhang_san);
    assert_eq!(zhang.name, None);
}

/// copy 类型
/// 简单类型 整数、字符等可以直接将值赋值给其它的变量
/// 在栈内存上分配空间
/// 赋值给其它变量会获得一份完整的拷贝
/// 赋值 copy 类型变量会得到一份完整的拷贝，而不是 move 它
///
/// ```
/// let s1 = "string_x";
/// let s2 = s1;
/// println!("s1 => {} s2 => {}", s1, s2);
///
/// let i1 = 10;
/// let i2 = i1;
/// println!("i1 => {} i2 => {} ", i1, i2);
///
/// let c1 = 'c';
/// let c2 = c1;
/// println!("c1 => {} c2 => {}", c1, c2);
/// }
/// ```
///
/// 标准的 Copy 类型包括所有的机器整数、浮点数、 char 和 bool 类型等
/// Copy 类型的元祖和定长数组也是 Copy 类型
/// 只有这种可以通过逐位赋值的类型可以称为 Copy 类型
///
/// ```
/// let copied_pair = int_pair;
/// println!("copied pair => {:?}", copied_pair);
///
/// let nums = [1, 2, 3, 4, 5];
/// let copied_nums = nums;
/// println!("copied nums => {:?}", copied_nums);
/// ```
///
/// 用户定义的类型，struct 默认是非 Copy 类型
/// 但是如果 struct 中所有的域都是 Copy 类型
/// 可以加上 #[derive(Copy, Clone)] 注解在 struct 定义上将其变为 Copy 类型
/// ```
/// #[derive(Copy, Clone)]
/// struct Person {age: i32};
/// let a = Person {age: 12};
/// let b = a;
/// ```
fn copy_types() {
    let s1 = "string_x";
    let s2 = s1;
    println!("s1 => {} s2 => {}", s1, s2);

    let i1 = 10;
    let i2 = i1;
    println!("i1 => {} i2 => {} ", i1, i2);

    let c1 = 'c';
    let c2 = c1;
    println!("c1 => {} c2 => {}", c1, c2);

    let int_pair = (10, 20);
    let copied_pair = int_pair;
    println!("copied pair => {:?}", copied_pair);

    let nums = [1, 2, 3, 4, 5];
    let copied_nums = nums;
    println!("copied nums => {:?}", copied_nums);

    #[derive(Copy, Clone)]
    struct Person {age: i32};
    let a = Person {age: 12};
    let b = a;
    println!("b.age => {}", b.age);
}

/// Rc 和 Arc 类型
/// 引用计数指针类型
/// Rc 和 Arc 唯一的区别是 Arc 是线程安全的
/// Rc<T> 的值是一个指向堆上分配的 T 的指针，以及一个指针计数
/// 克隆 Rc<T> 不会拷贝 T
/// 而是创建另一指向 T 的指针并将计数增加
/// 常规的所有权规则适用于指针本身
/// 当所有的 Rc 都丢弃掉了， Rust 也会丢弃调 String
fn rc_and_arc() {
    let s: Rc<String> = Rc::new("string".to_string());
    let t = s.clone();
    let u = s.clone();

    println!("s => {:?} t => {:?} u => {:?}", s, t, u);
}