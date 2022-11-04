use std::collections::HashMap;

fn main() { }

type Table = HashMap<String, Vec<String>>;

/// 因为 Rust move 特性
/// show_table 函数将会取得实参 table 的所有权
/// 在内部的第一重 for 循环中消费掉每一个 string 和 Vec<String>
/// 在第二重 for 循环中消费调每个 Vec<String> 中所有的元素
/// 函数结束后整个 table 值将会被丢弃
/// 为了避免这种问题，可以使用 引用 的特性
/// - 引用 访问一个值而不会影响它的所有权
fn show_table(table: Table) {
    for (artist, works) in  table {
        println!("Works by \"{}\" :", artist);
        for work in works {
            println!("{}; ", work);
        }
    }
}

#[test]
fn test_show_table() {
    let mut table = Table::new();
    table.insert("Li Bai".to_string(),
                 vec!["Jing Ye Si".to_string(), "Qiang Jin Jiu".to_string()]);
    table.insert("Du Fu".to_string(),
                 vec!["Wang Yue".to_string(), "Deng Gao".to_string()]);
    show_table(table);
}

/// [引用] 访问一个值而不会影响它的所有权
/// [共享引用] 可以读取但是不能更改所引用的值，对于一个值，可以有任意个共享引用
/// 共享引用是 [Copy] 类型的
/// ```
/// &t 是 t 的引用
/// 如果 t 的类型为 T
/// 则 &t 的类型为 &T
/// ```
/// [可变引用] 可变引用可以对引用的值进行读写操作，然而对于该值不能有任何其它类型的引用同时存在
/// 可变引用不是 [Copy] 类型
/// ```
/// &mut v 产生一个 v 的可变引用
/// 它的类型为 &mut T
/// ```
/// [可变引用]和[共享引用]的区别为在编译时强制使用[唯一写]和[多个读]规则的一种方式;
/// 一旦一个值存在[共享引用]，它的所有者都不能修改它；
/// 同理，一旦一个值存在[可变引用]，它会排斥对该值所有的访问，直到该可变引用失效；
/// 将[共享和修改完全分离]是保证内存安全的关键。
///
/// ```
/// fn show_table_reference(table: &Table);
/// ```
/// 不会修改 table 的内容，只是读取它的内容;
/// [引用是非拥有指针] 所以 table 变量仍然保留了整个结构的所有权;
/// show_table_reference 函数仅仅是[借用]了它。
///
fn show_table_reference(table: &Table) {
    for (artist, works) in table {
        println!("Works by \"{}\" :", artist);
        for work in works {
            println!("{}; ", work);
        }
    }
}

#[test]
fn test_show_table_reference() {
    let mut table = Table::new();
    table.insert("Li Bai".to_string(),
                 vec!["Jing Ye Si".to_string(), "Qiang Jin Jiu".to_string()]);
    table.insert("Du Fu".to_string(),
                 vec!["Wang Yue".to_string(), "Deng Gao".to_string()]);
    show_table_reference(&table);
    assert_eq!(table["Li Bai"][0], "Jing Ye Si".to_string());
}