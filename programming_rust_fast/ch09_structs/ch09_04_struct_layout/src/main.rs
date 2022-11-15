fn main() {
    let map = GrayscaleMap {pixels: vec![10, 20], size: (1024, 768)};
    println!("map addr => {:p}", &map);
    println!("map.pixels addr => {:p}", &map.pixels);
    println!("map.size addr => {:p}", &map.size);
}

/// 结构体在内存中的布局：
/// 在内存中，字段结构体和类元组结构体都是相同的事物，
/// 一组值的集合。
/// Rust 不会确保按照结构体或者元素的顺序在内存中对于结构体中的元素进行排序，
/// 但是，Rust 会将字段或者元素的值直接嵌套存储在结构本身的值当中。
struct GrayscaleMap {
    pixels: Vec<u8>,
    size: (usize, usize),
}