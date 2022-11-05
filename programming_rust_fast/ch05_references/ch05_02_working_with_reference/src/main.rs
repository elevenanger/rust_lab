fn main() {
    reference_and_dereference();

    assigning_references();

    references_to_references();

    comparing_references();
}

/// Rust 中的引用和解引用
/// - 在 Rust 中使用 & 操作符创建引用
/// - 使用 * 操作符显式进行解引用
/// ```
/// let a = 10;
/// let a_r = &a;
///
/// let mut b = 10;
/// let b_r = &mut b;
/// *b_r += 10;
/// ```
/// - [. 操作符] 可以隐式解引用左操作数
/// ```
/// struct Person {name: &'static str, age: i32 }
/// let ming = Person {name: "Ming", age: 21};
/// ```
///
/// - [.操作符] 隐式借用了左操作数的引用
/// ```
/// let mut v = vec![1, 3, 2];
/// (&mut v).sort();
/// println!("v => {:?}", v);
/// ```
fn reference_and_dereference() {
    let a = 10;
    let a_r = &a;
    println!("a => {}", *a_r);

    let mut b = 10;
    let b_r = &mut b;
    *b_r += 10;
    println!("b => {}", *b_r);

    struct Person {name: &'static str, age: i32 }

    let ming = Person {name: "Ming", age: 21};
    println!("ming.name => {} ming.age => {}", ming.name, ming.age);
    println!("ming.name => {} ming.age => {}", ming.name, ming.age);

    let mut v = vec![1, 3, 2];
    (&mut v).sort(); /* 隐式借用了左操作数的引用 */
    println!("v => {:?}", v);
}

/// 将引用赋值给变量使得变量指向一个新的地方
/// ```
/// let x = 10;
/// let y = 20;
/// let mut r = &x;
/// r = &y;
/// ```
/// r 首先指向 x ，赋值 &y 之后指向 y
fn assigning_references() {
    let x = 10;
    let y = 20;
    let mut r = &x;
    println!("r = {}", *r);
    r = &y;
    println!("r = {}", *r);
}

/// 引用的引用：
/// Rust 允许创建引用的引用，多重引用；
/// 无论多少重引用，
/// Rust 的 . 操作符都会自动解引用直到找到目标值。
/// ```
/// struct Person {name: String, age: u8}
/// let wong = Person {name: "Wong".to_string(), age: 12};
/// let wong_r = &wong;
/// let wong_rr = &wong_r;
/// let wong_rrr = &wong_rr;
/// println!("wong.name => {} wong.age => {}", wong_rrr.name, wong_rr.age);
/// ```
fn references_to_references() {
    struct Person {name: String, age: u8}
    let wong = Person {name: "Wong".to_string(), age: 12};
    let wong_r = &wong;
    let wong_rr = &wong_r;
    let wong_rrr = &wong_rr;
    println!("wong.name => {} wong.age => {}", wong_rrr.name, wong_rr.age);
}

/// 引用比较：
/// 和 . 操作符一样，Rust 比较运算符也能够对多重引用进行解引用。
/// ```
/// let x = 10;
/// let y = 11;
/// let rx = &x;
/// let ry = &y;
/// let rrx = &rx;
/// let rry = &ry;
///
// assert!(rrx < rry);
/// ```
/// - 比较运算符两边的操作数需要是相同类型的，包括引用类型（不能比较 &x 和 &&x）
/// ```
/// assert!(rx < *rry);
/// ```
fn comparing_references() {
    let x = 10;
    let y = 11;

    let rx = &x;
    let ry = &y;

    let rrx = &rx;
    let rry = &ry;

    assert!(rrx < rry);
    assert!(rx < *rry);
}

/// 引用不会为 NULL 值：
/// C 和 C++ 可以使用空指针表示没有值，
/// 在 Rust 中必须有一个引用指向的值，
/// 使用 Option<&T> 在机器层面，
/// Rust 使用 None 表示空指针，
/// 使用 Some(r) 表示 &T 类型值
/// Option<T> 和 C 和 C++ 中的空指针一样高效，
/// 而且这种用法还更安全，
/// 你需要在使用它的值之前检查它是否为空
fn reference_are_never_null() -> Option<&'static str> {
    "something";
    None
}