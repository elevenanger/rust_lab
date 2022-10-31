use text_colorizer::*;
fn main() {
    rust_fixed_width_numeric_type_summary();

    integer_literals();

    byte_literals();

    float_point_types();

    bool_type();

    characters();
}

/// Rust 中的固定宽度数值类型汇总
/// Rust 中的数值类型命名遵循一个简单的范式
/// 以位为单位的宽度和呈现形式
fn rust_fixed_width_numeric_type_summary () {

    print_desc("数值类型汇总：");

    println!("| {} | {} | {} | {} |",
                "bits".blue().bold(),
                "unsigned int".blue().bold(),
                "signed int".blue().bold(),
                "floating point".blue().bold());

    println!("| {:^4} | {:^12} | {:^10} | {:^14} |",
                8, "u18", "i8", " ");
    println!("| {:^4} | {:^12} | {:^10} | {:^14} |",
             16, "u16", "i16", " ");
    println!("| {:^4} | {:^12} | {:^10} | {:^14} |",
             32, "u32", "i32", "f32");
    println!("| {:^4} | {:^12} | {:^10} | {:^14} |",
             64, "u64", "i64", "f64");
    println!("| {:^4} | {:^12} | {:^10} | {:^14} |",
             128, "u128", "i128", "");
    // sys 表示系统位数 32 或者 64 位
    println!("| {:^4} | {:^12} | {:^10} | {:^14} |",
             "sys", "usize", "isize", "");
}

/// 整数字面量
fn integer_literals() {
    print_desc("整数字面量：");
    println!("数字后面添加类型后缀   => 32i8         = {}",
             32i8.to_string().blue().bold());
    println!("十六进制表示           => 0xf          = {}",
             0xf.to_string().blue().bold());
    println!("八进制表示             => 0o11         = {}",
             0x11.to_string().blue().bold());
    println!("二进制表示             => 0b1111_1111  = {}",
             0b1111_1111.to_string().blue().bold());
}

/// 字节字面量
/// 表示类字符字面量的 u8 类型值
/// b'X' 表示字符 X 的 ASCII 码
fn byte_literals() {
    print_desc("字节字面量");
    println!("X     => {} = {}", 'X', b'X');
    println!("单引号 => {} = {}", '\'', b'\'');
    println!("反斜杠 => {} = {}", '\\', b'\\');
    println!("换行符 => {} = {}", '\n', b'\n');
    println!("回车符 => {} = {}", '\r', b'\r');
    println!("制表符 => {} = {}", '\t', b'\t');
}

/// 浮点类型
fn float_point_types() {
    print_desc("浮点类型:");
    /*
    浮点类型表示法
    浮点类型数字整数部分后面的部分是可选的
    但是至少要有小数部分、指数或者类型后缀中的一种
    以便和整数类型区分开来
    如果浮点类型数字缺乏后缀
    Rust 会从上下文来检查值是如何被使用的
    如果最终检查发现 f32 或者 f64 浮点类型都可以匹配得上
    默认会使用 f64 类型
     */
    println!("3. = {}", 3.);
    println!("3f32 = {}", 3f32);
    println!("f32 min => {} f32 max => {}",
             f32::MIN, f32::MAX);
    println!("f64 min => {} f64 max => {}",
             f64::MIN, f64::MAX);
}

/// 布尔类型
fn bool_type() {
    print_desc("布尔类型：");
    /*
    Rust 对于 if 和 while 等语句的控制结构要求非常严格
    必须要是布尔表达式
    布尔值可以转换为整数类型
    尽管只需要一位就可以表示布尔值
    但是 Rust 使用了一个字节来存储布尔值
    所以可以创建指向布尔值的指针
     */
    println!("false as i32 => {}", false as i32);
    println!("true as i32 => {}", true as i32);
}

/// 字符类型
/// Rust 字符类型 char 表示单个 Unicode 字符 32 位值
/// Rust 对于单个字节使用 char 类型
/// 但是对于 String 和文本流使用的是 UTF-8 编码
/// String 中的文本是以一系列的 UTF-8 字节来表示的，而不是一串字符类型
fn characters() {
    print_desc("字符：");
    println!("a => {}", 'a');
    /*
    使用 Unicode 的十六进制码点来表示字符
    如果字符的码点在 U+0000 - U+007F 之间可以直接使用 '\xHH' 的形式表示
    HH 是两个十六进制数字
    也可以使用'\u{HHHHHH}' 的形式来写
    Unicode 十六进制码点最多6位长度
     */
    println!("{}", '\x2A');
    println!("{}_{}", '\u{CA0}', '\u{CA0}');
    println!("* as i32 => {}", '*' as i32);
    println!("{}", '*'.is_alphabetic());
}

fn print_desc(desc: &str) {
    println!("\n{}", desc.green().bold());
}