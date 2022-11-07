fn main() {
    borrowing_local_variable();

    receiving_references_as_function_arguments();

    passing_references_to_functions();

    returning_references();

    struct_containing_references();

    distinct_lifetime_parameters();

    omitting_lifetime_parameters();
}

/// 借用局部变量：
/// - 约束1：对变量的引用不能超过变量本身的生命周期（最大生命周期）,
/// 变量的生命周期必须包括借用该变量的引用。
/// - 约束2：引用的生命周期必须包含它所赋值的变量的生命周期（最小生命周期）。
/// ```
/// let v = vec![1, 2, 3];
/// {
///     let r = &v[1];
///     assert_eq!(r, 2);
/// }
/// println!("{:?}", v);
/// ```
/// v 的生命周期到 println! 宏结束；
/// r 的声明周期在 assert_eq! 调用结束；
/// 对变量 v[1] 引用 &v[1] 的生命周期和 v 的一样；
/// r 是 &v[1] 赋值的变量；
/// v 的生命周期包含 &v[1];
/// &v[1] 的生命周期包含 r 的生命周期；
fn borrowing_local_variable() {
    let v = vec![1, 2, 3];
    {
        let r = &v[1];
        assert_eq!(*r, 2);
    }
    println!("{:?}", v);
}

/// 接收引用作为函数的实参：
fn receiving_references_as_function_arguments() {
    let r = &256;
    f(r);
    unsafe {
        assert_eq!(*STASH, 256);
    }

    static WORTH_POINTING_AT: i32 = 1000;
    f(&WORTH_POINTING_AT);
    unsafe {
        assert_eq!(*STASH, 1000);
    }
}

/// Rust 中与全局变量等价的特性为 static ；
/// 它的值在程序启动时创建并且一直持续到程序结束；
/// 和其它的声明一样：
///  Rust 的模块系统控制 static 变量的可见性，它们仅仅在它们的生命周期中是全局的，
/// 而不是可见性
/// - 每个 static 变量都需要进行初始化；
/// - 可变的 static 变量本质上是非线程安全的，所以只能通过 unsafe 代码块进行访问。
static mut STASH: &i32 = &128;

/// ```
/// p: &'static i32
///```
/// 'static 中的 ' 表示这个是函数 f 的声明周期参数;
/// 参数的默认生命周期为 'a 表示任意的生命周期；
/// 因为函数内访问的 STASH 是全局变量，拥有 static 声明周期；
/// 函数 f 的参数需要对 static 生命周期的全局变量进行修改；
/// 则参数 p 也必须是 static 生命周期才会保证 STASH 全局变量不会在函数执行完成之后变成悬空指针；
/// ['static] 生命周期参数表示函数的参数是 static 生命周期的。
fn f(p: &'static i32) {
    unsafe {
        STASH = p;
    }
}

/// 调用者将引用作为参数传递给函数：
fn passing_references_to_functions() {
    let mut x = 10;
    g(&mut x);
    assert_eq!(x, 100);
}

fn g(p: &mut i32) {
    *p = *p * 10;
}

/// 返回引用
fn returning_references() {
    let numbers = [100, 10, 123, 213];
    println!("smallest => {}", *smallest(&numbers));
}

fn smallest(v: &[i32]) -> &i32 {
    let mut s = &v[0];
    for r in &v[1..] {
        if *r < *s {
            s = r;
        }
    }
    s
}

/// 数据结构中的引用域：
/// ```
/// struct S<'a> {r: &'a i32}
/// struct D {r: S<'static>};
/// ```
/// 在类型定义中出现引用类型，必须写出它们的生命周期。
fn struct_containing_references() {
    struct S<'a> {r: &'a i32}
    struct D<'a> {r: S<'a>}

    {
        let x = 10;
        let s = S {r: &x};
        assert_eq!(*s.r, 10);
    }

    {
        let ss = S {r: &1000};
        let d = D {r: ss};
        assert_eq!(*d.r.r, 1000);
    }
}

/// 区分生命周期参数：
/// ```
/// struct S<'a, 'b> {
///     x: &'a i32,
///     y: &'b i32
/// }
///
/// fn f_1<'a, 'b>(r: &'a i32, s:&'b i32) -> &'a i32 {
///     r
/// }
/// ```
/// 结构 struct S<'a, 'b> 中的元素具有 a b 两种不同的生命周期；
/// Rust 自动选择满足条件的最小的 a b 生命周期。
fn distinct_lifetime_parameters() {
    let r = 11;
    {
        let s = 10;
        let s1 = S1 {x: &r, y: &s};
        let re = f_1(s1.x, s1.y);
        println!("{}", *re);
    }
}

struct S1 <'a, 'b> {
    x: &'a i32,
    y: &'b i32
}

fn f_1<'a, 'b>(r: &'a i32, s:&'b i32) -> &'a i32 {
    r
}

/// 省略生命周期参数：
/// 只有在必须要写出生命周期的场景下才需要显示写出生命周期，
/// Rust 会自动在每个需要生命周期的地方赋予一个单独的生命周期，
/// 但是如果函数是某个类型的方法，接收 self 作为参数，
/// Rust 会将 self 的生命周期作为所有返回值的生命周期，
/// Rust 假定无论借用什么，都是从 self 借用。
/// ```
/// fn find_by_prefix(&self, prefix: &str) -> Option<&String>
/// fn find_by_prefix(&'a self, prefix: &'b str) -> Option<&'a String>
/// ```
fn omitting_lifetime_parameters() {
    let table = StringTable {elements:
        vec!["anger-1".to_string(),
             "anger-2".to_string(),
             "eleven-1".to_string()] };

    assert_eq!(Some(&"anger-1".to_string()), table.find_by_prefix("anger-"));
    assert_eq!(Some(&"eleven-1".to_string()), table.find_by_prefix("eleven-"));
    assert_eq!(None, table.find_by_prefix("eleven-anger"));
}

struct StringTable {
    elements: Vec<String>,
}

impl StringTable {
    fn find_by_prefix(&self, prefix: &str) -> Option<&String> {
        for i in 0..self.elements.len() {
            if self.elements[i].starts_with(prefix) {
                return Some(&self.elements[i])
            }
        }
        None
    }
}