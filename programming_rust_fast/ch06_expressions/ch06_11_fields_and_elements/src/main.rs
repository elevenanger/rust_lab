fn main() {
    struct_fields();

    tuple_elements();

    range_operator();
}

/// 结构中的域：
/// 通过 . 操作符获取结构中的域,
/// name.field_name
fn struct_fields() {
    struct Person {name: String, age: u8}

    let wong = Person {name: "wong".to_string(), age: 18};
    println!("name => {} age => {}", wong.name, wong.age);
}

/// 元组中的元素：
/// 通过 . 操作符和元素索引获取元组中的元素
/// name.index
fn tuple_elements() {
    let banana_info = ("Banana", 10.50, '¥', "Guang Dong");
    println!("{} {} price : {} {}",
                banana_info.3, banana_info.0,
                banana_info.2, banana_info.1);
}

/// 范围运算符：
/// .. 范围运算符，用来表示一个区间,
/// .. 操作符允许省略两边的操作数：
/// .. 全部范围；
/// a .. 从 a 开始直到结束的范围；
/// .. b 从 0 开始直到 b 结束；
/// a .. b 从 a 开始到 b 结束。
/// .. 是半开半闭区间，结束值不包括在范围中。
///
/// ..= 操作符产生一个封闭区间，结束值包括在范围内。
fn range_operator() {
    let a: Vec<i32> = (0..10).collect();
    let b: Vec<i32> = (0..=10).collect();
    println!("a => {:?}", a);
    println!("b => {:?}", b);
}