fn main() {}

/// 类元组结构：
/// 构造类元祖结构的方式和构造元组的方式类似，除了必须声明类型名。
/// ```
/// let image_bounds = Bounds (1024, 768);
/// assert_eq!(image_bounds.0, 1024);
/// assert_eq!(image_bounds.1, 768);
/// ```
/// 类元祖结构中保存的值成为元素，
/// 和元组一样，
/// 通过元素的索引访问元素。
/// 每个元素可以但是声明为 public 。
/// 在最基础的层面，字段命名结构和类元组结构是非常相似的，
/// 选择使用那种结构取决于易读性、歧义性和简洁性，
/// 如果更多使用 . 运算符来获取值，
/// 通过名称来识别字段可以为代码读者提供更多的信息，
/// 并且可能使得代码类型更为健壮；
/// 但是如果更经常使用模式匹配来匹配元素，
/// 类元组结构更适用。
pub struct Bounds (usize, usize);

#[test]
fn test_bounds() {
    let image_bounds = Bounds (1024, 768);
    assert_eq!(image_bounds.0, 1024);
    assert_eq!(image_bounds.1, 768);
}

/// 类元组结构适用于创建新类型，
/// 拥有单一元素的结构，
/// 但是需要进行更加严格或者个性化的格式校验。
/// 比如可以定义一个 Ascii 类型，
/// 使用这个类型比单纯使用 Vec<u8> 具有更好的表达能力。
struct Ascii(Vec<u8>);

#[test]
fn test_ascii() {
    let plus = Ascii(vec![0x2B]);
    let s = String::from_utf8_lossy(&plus.0);
    println!("s => {}", s);
}