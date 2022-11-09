fn main() {
    function_call();

    method_call();

    type_associate_function();
}

/// 函数调用：
fn function_call() {
    let product = product(100, 200);
    println!("{}", product);
}

fn product(x: i32, y: i32) -> i32 {
    x * y
}

/// 方法调用：
/// Rust 通常会在引用和它引用的值之间做出鲜明的区分，
/// 使用 . 操作符, Rust 会按需进行解引用或者借用引用，
/// 所以 T 类型或者 T 类型的引用 &T 或者智能指针 Box<T> Rc<T>,
/// 进行方法调用能够产生相同的效果。
fn method_call() {
    let s = "test".to_string();
    println!("len => {}", &s.len());
    println!("len => {}", s.len());
}

/// 类型相关函数：
/// 类型相关函数类似于面向对象语言中的静态方法，
/// 常规的方法调用是通过值调用，
/// 类型函数是通过类型调用:
/// T::fn()
/// ```
/// let mut v = Vec::<i32>::new();
/// ```
/// rust 泛型参数的语法 ::<T> ,
/// ::<...> 符号是 Rust 中的 '涡轮鱼' 符号,
/// 在 Rust 可以进行类型推断的场景下可以省略类型符号。
fn type_associate_function() {
    let mut v = Vec::<i32>::new();
    v.push(1);
    v.push(2);

    println!("{:?}", v);
}