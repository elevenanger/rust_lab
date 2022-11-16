fn main() { }

/// 为 struct 实现通用的 trait:
/// 在 Rust 中，每个特性都有一个名称，
/// Copy, Clone, Debug, PartialEq,
/// 这些都是 trait,
/// 对于这些标准的 trait,
/// 不需要手写实现，除非需要一些定制化的行为，
/// Rust 可以自动生成相应的实现，
/// 加上 #[derive()] 注解即可。
#[derive(Copy, Clone, Debug, PartialEq)]
struct Point {
    x: f64,
    y: f64
}

#[test]
fn test_point() {
    let p = Point {x: 10.0, y: 100.0};
    let q = p.clone();
    println!("p => {:?}", p);
    println!("q => {:?}", q);
}
