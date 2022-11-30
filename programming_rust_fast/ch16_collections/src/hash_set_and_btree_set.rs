use std::collections::{BTreeSet, HashSet};

/// set 是方便快速查找的元素的集合，
/// set 中的元素不重复
#[test]
fn test_hash_set() {
    let mut set:HashSet<i32> = (0..=10_000).collect();
    assert!(set.contains(&9999));
    set.insert(10);
    assert_eq!(set.len(), 10_001);
    // get 获取元素
    assert_eq!(set.get(&10), Some(&10));
    // take 获取并移除元素
    assert_eq!(set.take(&10), Some(10));
    assert_eq!(set.len(), 10_000);
    // replace 值不存在则执行 insert 值已存在则将新的值替换旧的值，返回旧值
    assert_eq!(set.replace(-1), None);
    assert_eq!(set.replace(-1), Some(-1));
}

#[test]
fn test_whole_set_operations() {

    let print_series = |i| print!("{} ", i);

    let a:BTreeSet<i32> = (0..=10).collect();
    let b:BTreeSet<i32> = (5..=15).collect();
    println!("a => {:?} \nb => {:?}", &a, &b);
    println!("\na intersection b:");
    a.intersection(&b).for_each(print_series);
    println!("\na union b:");
    a.union(&b).for_each(print_series);
    println!("\na difference b:");
    a.difference(&b).for_each(print_series);
    println!("\nb difference a:");
    b.difference(&a).for_each(print_series);
    println!("\na symmetric difference b:");
    a.symmetric_difference(&b).for_each(print_series);
    println!("\nb symmetric difference a:");
    b.symmetric_difference(&a).for_each(print_series);

    // is_disjoint 两个集合没有相同的元素返回 true
    assert!(!a.is_disjoint(&b));
    // a.is_subset(&b) a 是 b 的子集返回 true
    assert!(!a.is_subset(&b));
    // a.is_superset(b) a 是 b 的超集返回 true
    assert!(!a.is_superset(&b));
}