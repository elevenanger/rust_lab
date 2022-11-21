use std::ops::{Index, IndexMut};
use std::collections::HashMap;

#[test]
fn test_hash_map_index() {
    /*
    HashMap 可以使用任何可哈希的类型或者排序的类型作为索引,
    ```
    assert_eq!(chinese_count["十"], 10);
    assert_eq!(*chinese_count.index("千"), 1_000);
    ```
    hashmap[index] 和 *hashmap.index(index) 本质是一样的，
    它实现了 Index trait.

    Output 指定了 index() 方法返回的类型，
    在下面的例子中，返回的类型为 i32
     */
    let mut chinese_count = HashMap::new();
    chinese_count.insert("十", 10);
    chinese_count.insert("百", 100);
    chinese_count.insert("千", 1_000);
    chinese_count.insert("万", 10_000);

    assert_eq!(chinese_count["十"], 10);
    assert_eq!(*chinese_count.index("千"), 1_000);
}

#[test]
fn test_mutable_indexed_sequence() {
    /*
    IndexMut 继承了 Index trait ，
    它的方法接收一个 &mut self 作为参数
    并且返回一个 Output 值的可变引用。
     */
    let mut a = ["you".to_string(), "are".to_string(),
        "the".to_string(), "o".to_string()];
    println!("{:?}", a);

    a[3].push_str("ne");
    a.index_mut(3).push_str(".");
    println!("{:?}", a);
}