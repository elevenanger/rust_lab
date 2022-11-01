use text_colorizer::*;

fn main() {
    rust_fixed_width_numeric_type_summary();

    integer_literals();

    byte_literals();

    float_point_types();

    bool_type();

    characters();

    tuples();

    pointer_types();

    sequence_of_values();

    string_types();
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

/// 元组是由多种不同类型值组成的数据结构
/// 元组的写法是一系列通过 , 分割的元素
/// t = ("china", 1993) 是一个元组，它的类型是 (&str, i32)
/// 使用数字来访问元组中的元素
/// t.0 t.1
/// 元组仅允许使用常量作为索引 t.0
/// 不能使用变量 t.i
fn tuples() {
    print_desc("元组:");
    let text = "Anger China";
    let (name, country) = text.split_at(5);
    println!("name => {}\tcountry => {}", name.bold(), country);
    let person = text.split_at(5);
    println!("name => {}\tcountry => {}", person.0.bold(), person.1);
}

/// 指针类型
/// Rust 的设计初衷之一是保持最小的内存分配
/// 默认情况下，值嵌套
/// ((0, 0), (10, 100)) 的值存储为 4 个相邻的整数
/// 如果将它的值存储在一个局部变量中
/// 局部变量则为四个整数的宽度
/// 在堆上不分配空间
/// 这极大提升内存管理效率
/// 但是如果 Rust 程序中需要一个值指向另一个值
/// 需要显式的使用指针类型
/// 这里将讨论三种指针 Reference Box Pointer
fn pointer_types() {
    print_desc("指针类型:");

    references();

    boxes();

    raw_pointers();
}

/// &String 的值是对于一个 String 值的引用
/// 引用是 Rust 的基本指针类型
/// 在运行时，对于一个 i32 值的引用是一个保存 i32 值的地址的单独的机器字
/// 它可能在堆内存或者栈内存上
/// &x 表达式产生一个对于 x 的引用
/// 在 Rust 术语中，我们称之为将它的引用借用给了 x
/// 给与一个引用 r , *r 表达式指向 r 的值
/// 与 c 指针不同的是
/// Rust 引用永远不会为 null
/// Rust 会持续追踪值的生命周期和所有权
/// Rust 的指针有两种风格
/// &T
/// 一个不可变的共享指针
/// 对于一个值在同一时间可以有多个共享指针，但是这些指针都是只读的
/// &mut T
/// 一个可变的，独占指针
/// 可以对指针引用的值进行读写操作
/// 但是只要存在独占指针，就不能再对这个值有其它类型的引用
/// 只能通过这个引用访问这个值
/// 在共享和可变引用之间，Rust 采用二分法来强制执行"唯一写或者多个读"规则：
/// 要么一个地方可以读写一个值，要么在任意人之间共享读权限，但是不能二者兼有
/// 这种区分方式，在编译期间强制进行检查，这个 Rust 进行安全保证的核心
fn references() {
    print_desc("1.引用类型(Reference)：");
    let i = 31;
    let mut j = 32;

    let i1 = &i;
    let i2 = &i;
    let j1 = &mut j;
    println!("i = {}, i1 = {}, i2 = {}", i, *i1, *i2);
    /* j 的值只能通过 j1 进行访问 */
    println!("j1 = {}", *j1);
    *j1 = 33;
    println!("j1 = {}", *j1);
}

/// 装箱类型(Box)
/// 在堆上分配一个值最简单的方式就是使用 Box::new
///
/// #Examples
/// '''
/// let i = 1;
/// let b = Box::new(i);
/// '''
/// 当 b 离开作用域范围，内存会立即释放
/// 除非 b 已经被移动，例如在函数中间返回它
/// 移动是 Rust 处理堆上分配的值的关键
fn boxes() {
    print_desc("封装(Box):");
    let name = ("wong", "wu");
    let name_b = Box::new(name);
    println!("name => {:?}", name_b);
}

/// 原始指针
/// 使用原始指针是不安全的
/// 因为 rust 不会追踪它的指向
/// 只能在 unsafe 代码块中对原始指针解引用
fn raw_pointers() {
    let a = 10;
    let mut b = 20;

    let a_raw = &a as *const i32;
    unsafe {
        println!("a = {}, b = {}", *a_raw, b);
    }

    let b_mut_raw = &mut b as *mut i32;
    unsafe {
        *b_mut_raw = 10;
        println!("b = {}", *b_mut_raw);
    }
}

/// Rust 中有三种表示一系列值的数据结构
///
/// type[T, N] 表示有 N 个值的数组：
/// 数组的大小是一个在编译期决定的常量并且是这个类型的一部分
/// 无法扩大或者缩小数组的大小
///
/// type Vec<T> 表示类型 T 的矢量数组
/// 是一个动态分配、可以增长的 T 类型序列
/// 矢量数组中的元素在堆内存上，可以按照需要调整矢量数组的大小
/// 新增新的元素、将另外一个矢量数组追加在前一个矢量数组之后或者删除元素等等
///
/// types &[T] 和 types &mut [T] 称为 T 类型的共享切片和 T 类型的可变切片
/// 是某些值中部分元素的引用
/// 就像矢量或者数组
/// 可以将切片看成指向第一个元素的指针
/// 以及从这个点开始可以访问的元素数量
/// 和指针一样
/// 可变切片可以访问以及修改元素，但是不能共享
/// 共享切片可以在多个访问者之间共享但是不能修改元素
///
/// 三种类型中任意一种类型的值 v
/// v.len() 返回元素的数量
/// v[i] 指向 v 中第 i 个元素
/// 第一个元素是 v[0]
/// 最后一个元素是 v[v.len - 1]
/// i 必须是 usize 类型值
///
fn sequence_of_values() {

    print_desc("元素序列类型：");

    arrays();

    vectors();

    slices();
}

/// 数组
/// 声明数组的方式
/// 显式声明数组的长度和元素类型，可以在声明数组的地方赋值
/// let name: [type; size] = [...]
/// 不显示声明数组的长度和元素类型，使用符合表达式在声明的时候赋值，Rust 自动进行类型推断
/// let name = [...]
/// value 是每个元素的值，size 是数组的大小
/// let name = [value; size]
fn arrays() {
    print_desc("数组：");
    let mut numbers: [u32; 5] = [1, 2, 4, 5, 3];

    for num in numbers {
        print!("{} ", num);
    }
    println!();

    numbers.sort();
    for num in numbers {
        print!("{} ", num);
    }
    println!();

    let countries =  ["China", "Japan", "American", "Chili"];
    for i in 1..3 {
        println!("{}", countries[i]);
    }

    let mut sieve = [true; 1000];
    for i in 2..100 {
        if sieve[i] {
            let mut j = i * i;
            while j < 10000 {
                sieve[i] = false;
                j += i;
            }
        }
    }

    assert!(sieve[1]);
    assert!(!sieve[4]);

}

/// 矢量数组
///
/// 矢量数组 Vec<T> 由三个值组成
/// 1、一个指向堆内存上分配的缓冲区的指针，它是由 Vec<T> 创建并拥有
/// 2、这个缓冲区能够容纳的元素数量
/// 3、当前数组实际的元素数量
/// 当缓冲区达到了容量上限，添加新的元素需要更大的缓冲区
/// 拷贝现有的缓冲区内容到新的缓冲区
/// 更新当前矢量数组的指针和容量来对应相应新的缓冲区
/// 最终释放掉旧的缓冲区
/// 如果事先知道 vector 的元素数量
/// 可以使用 Vec::with_capacity 从一开始就创建足够大的缓冲区来保存所有的元素
/// 如果超过预估的上限， vector 也会和常规一样扩展缓冲区的大小
///
/// 创建矢量数组的方式：
///
/// 使用 vec! 宏创建矢量数组
/// let nums = vec![1, 2, 3, 4]
///
/// 创建一个长度为 n 的，值都为 v 的数组
/// vac![v; n]
///
/// 使用 Vec::new() 函数创建一个空 vector
/// Vec::new()
///
/// 从迭代器产生的值中创建数组
/// let v: Vec<i32> = (0..n).collect()
fn vectors() {
    print_desc("矢量数组：");
    let nums = vec![1, 2, 3];
    for num in nums.iter() {
        print!("{} ", num);
    }
    println!();

    let nums_2 = vec![100; 10];
    for num in nums_2 {
        print!("{} ", num);
    }
    println!();

    let mut nums_3:Vec<i32> = (0..10).collect();
    /* push(v) 在 vector 末尾追加 v */
    nums_3.push(10);
    nums_3.push(100);
    /* reverse() 翻转 vector */
    nums_3.reverse();
    nums_3.remove(2);
    for num in nums_3 {
        print!(" {}", num);
    }
    println!();

    let mut nums_4 = Vec::with_capacity(10);
    nums_4 = (10..20).collect();
    /* insert(index, v) 将 v 插入到 index 位置 */
    nums_4.insert(3, 100);

    for num in nums_4.iter() {
        print!("{} ", num);
    }
    /* 使用 pop() 方法返回并删除最后一个元素 */
    assert_eq!(nums_4.pop(), Some(19));
}

/// 切片
/// 不需要声明长度，是数组或者矢量数组中的一个区域
/// slice 可以是任意长度，可以直接使用变量进行存储
/// 或者通过函数参数进行传递
/// slice 通过引用进行传递
/// slice 的引用是一个胖指针 包含指向切片中第一个元素的指针以及切片中元素的数量
fn slices() {
    print_desc("切片：");
    let v = vec![1, 2, 3, 4, 5];
    let a = [1, 2, 3, 4, 5];

    /* 如果没有声明切片的数量
        Rust 自动将 vector 或者数组的长度赋给它 */
    let sv: &[i32] = &v; /* sv: &[i32; 5] */
    let av: &[i32] = &a; /* av: &[i32; 5] */

    /* 对切片的引用是指向内存中一系列连续的值的无所属的指针
        因为这个特性可以很方便地编写一些函数能够同时适用于数组和矢量 */
    print_seq(sv);
    print_seq(av);
    print_seq(&sv[2..4]);
}

fn print_seq(seq: &[i32]) {
    for num in seq {
        print!("{} ", num);
    }
    println!();
}

/// 字符串类型
fn string_types() {
    print_desc("字符串类型：");

    string_literal();

    byte_string();

    strings_in_mem();

    string();

    using_strings();
}

/// 字符串字面量
/// 使用 "..." 双引号包围的内容是字符串的字面量
/// Rust 还提供 raw string 可以不需要写反斜杠
/// 如果没有逃生序列
/// r"..." 但是在 raw string 中不能使用双引号
/// r##"..."## ## 是逃生序列
fn string_literal() {
    print_desc("字符串字面量：");
    let str = "\"Oh\", you are here.\n";
    print!("{}", str);
    let path = r"\wind\c\folder";
    println!("{}", path);
    let str_2 = r##"she said : "ok"."##;
    println!("{}", str_2);
}

/// 字节串
/// 字符串字面量以 b 为前缀的是字节串
/// 字节串是 u8 值的切片
/// raw byte string 以 br 开头
fn byte_string() {
    print_desc("字节串");
    /* method 的类型是 &[u8. 3]
        三个字节的数组的引用(切面) */
    let method = b"GET";
    assert_eq!(method, &[b'G', b'E', b'T']);
    println!("{}", method[1]);

    let raw_byte_str = br##"raw byte"##;
    println!("{}",  raw_byte_str[3]);
}

/// 字符串在内存中的分布
fn strings_in_mem() {
    print_desc("内存中的字符串：");
    /* String 类型 */
    let hello = "hello".to_string();
    /* &str 类型
       指向任意 String 的切片 */
    let ello = &hello[1..];
    /* len() 函数返回的是字符串的字节长度，而不是字节数量 */
    println!("hello.length => {}", hello.len());
    println!("hello.chars().count() => {}", hello.chars().count());
}

/// String 类型
/// 创建 String 的方式
/// .to_string() 方法将 &str 转换为 String
/// format!() 宏返回一个 String
fn string() {
    print_desc("String 类型：");
    let name = "Anger".to_string();
    let birth = format!("{}{}{}", "1993", "08", "xx");
    println!("name => {} birth =>{} ", name, birth);
}

/// string 的一些常用用法
fn using_strings () {
    print_desc("使用 string 类型：");
    let one = "ONE".to_string();
    println!("{}", one.to_lowercase()); /* 大小写转换 */
    /* 使用 != 或者 == 判等操作符对 string 进行比较 */
    assert_eq!(one.to_lowercase() == "one", true);
}

fn print_desc(desc: &str) {
    println!("\n{}", desc.green().bold());
}