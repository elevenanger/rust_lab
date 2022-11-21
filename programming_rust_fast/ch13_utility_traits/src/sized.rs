use std::fmt::Display;

/// Sized 类型：
/// Sized 类型的值都拥有相同的内存大小，
/// Rust 中大部分的类型都是 Sized 类型，
/// 即使是枚举也是一样，
/// 无论存在多少的变体，
/// 枚举类型的值始终占据足够存放最大的值的空间，
/// Vec<T> 也是 Sized 类型，
/// 它们在堆上分配的内存空间可以是可变的，
/// 但是在栈内存上面的指针大小是固定的。
///
/// 所有 Sized 类型都实现 std::marker::Sized trait
/// 它没有方法或者关联类型，
/// Rust 自动为应用它的所有类型进行相应的实现，
/// Sized 的唯一用途就是用于类型限定 T: Sized
/// 限制 T 类型只能是 Sized 类型，需要在编译时明确它的大小，
/// 这种类型的 trait 为标记 trait ，
///
/// Rust 存在一些 unsized 类型，
/// 它们值的空间不总是相同的大小，
/// 比如字符串字面量 str ,
/// 字符数不同占据的内存空间也就不同，
/// slice 类型 [T] 是 unsized ,
/// 一个指向 slice 的共享引用 &[T] 是 sized 类型，
/// 因为指针的大小是已知的，
/// trait 对象也是 unsized ，
/// 在编译期间无法确定其大小，
/// Rust 不能将 unsized 值存储在变量中或者作为实参传递，
/// 只能通过指针处理它们， 比如 &str Box<dyn write>,
/// 指向 unsized 的指针都是胖指针，
/// 占据两个机器字的宽度，
/// 指向 slice 的指针还带有 slice 的长度，
/// 指向 trait 对象的指针还带有一个指向类型方法实现的 vtable 的指针。
///
/// Rust 泛型的类型变量是默认是 Sized 不需要显式地声明它，
/// 如果必须要 Rust 的这个约束，
/// 可以显式地将参数声明为 unsized <T: ?Sized> ,
/// 表示 T 类型不需要是 Sized 类型,
/// 可以是 Sized 类型，也可以不是，
/// Box<T> 是一个胖指针。
///
/// 除了 slice 和 trait 对象之外，还有一种 unsized 类型，
/// 有且只有最后一个字段为 unsized 类型的结构，
/// 这种结构本身就是 unsized
/// ```
/// struct RcBox<T: ?Sized> {
///     ref_count: usize,
///     value: T
/// }
/// ```
/// 不能直接创建 RcBox 的值，
/// 首先要创建一个 sized 类型的 RcBox 的值 -> RcBox<String> ,
/// 因为 String 实现了 Display trait ，
/// 然后 Rust 允许将 &RcBox<String> 转换成一个胖指针 -> &RcBox<dyn Display> ,
/// 这个转换在将值作为实参传递的时候由 Rust 隐式地进行。
/// ```
/// let boxed_lunch: RcBox<String> =
///     RcBox {ref_count: 1, value: "lunch".to_string()};
/// let boxed_display:  &RcBox<dyn std::fmt::Display> = &boxed_lunch;
/// print_things(&boxed_display);
/// ```
fn print_things(boxed: &RcBox<dyn Display>) {
    println!("for you enjoyment {}", &boxed.value);
}

struct RcBox<T: ?Sized> {
    ref_count: usize,
    value: T
}

#[test]
fn test_rc_box() {
    let boxed_lunch: RcBox<String> =
        RcBox {ref_count: 1, value: "lunch".to_string()};
    let boxed_display:  &RcBox<dyn Display> = &boxed_lunch;
    print_things(&boxed_display);
}