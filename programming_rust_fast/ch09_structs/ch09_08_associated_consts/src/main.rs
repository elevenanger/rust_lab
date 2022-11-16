fn main() {
}

/// 二元坐标系
struct Vector2 {
    x: f32,
    y: f32,
}

impl Vector2 {
    /// 类型关联常量：
    /// 关联常量是常量值，
    /// 通常用于指定特定类型的常用值。
    /// 这些值与类型本身相关联，
    /// 可以直接使用他们而不需要类型的实例值，
    /// 和关联函数一样，通过类型名和 :: 操作符进行访问。
    const ZERO: Vector2 = Vector2 {x:0.0, y:0.0};
    const UNIT: Vector2 = Vector2 {x:1.0, y:1.0};
}

#[test]
fn test_vector2() {
    assert_eq!(Vector2::ZERO.x, 0.0);
    assert_eq!(Vector2::UNIT.y, 1.0);
}