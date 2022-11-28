use std::cmp::Ordering;
use std::collections::HashMap;
use std::error::Error;
use std::str::FromStr;

/// 简单累积运算
#[test]
fn test_simple_accumulation() {
    let a = [1, 2, 3];
    // 求和
    let sum: i32 = a.iter().by_ref().sum();
    assert_eq!(sum, 6);
    // 计数
    let count = a.iter().by_ref().filter(|x| *x % 2 == 0).count();
    assert_eq!(count, 1);
    // 求积
    let product: i32 = a.iter().by_ref().map(|x| x + 1).product();
    assert_eq!(product, 24);
}

/// max 和 min 方法比较的元素必须实现了 std::cmp::Ord trait
#[test]
fn test_min_and_max() {
    let a = [2, 3, 4, 5, -6];
    assert_eq!(a.iter().by_ref().max(), Some(&5));
    assert_eq!(a.iter().by_ref().min(), Some(&-6));
}

/// max_by min_by 根据所提供的比较函数返回最大和最小值
#[test]
fn test_max_by_and_min_by() {
    let cmp = |x: &i32, y: &i32| -> Ordering {
        x.abs().partial_cmp(&y.abs()).unwrap()
    };

    let a = [1, 2, 3, -4];
    assert_eq!(a.iter().copied().max_by(cmp), Some(-4));
    assert_eq!(a.iter().copied().min_by(cmp), Some(1));
}

#[test]
fn test_max_by_key_min_by_key() {
    let mut students = HashMap::new();
    students.insert("An", 100);
    students.insert("Ma", 48);
    students.insert("Gang", 77);
    students.insert("Ping", 97);

    assert_eq!(students.iter().max_by_key(|&(_name, grade)| grade), Some((&"An", &100)));
    assert_eq!(students.iter().min_by_key(|&(_name, grade)| grade), Some((&"Ma", &48)));
}

/// 可以使用比较运算符直接比较 string、vector、slice
/// 只要其中每个元素都是可比较的
#[test]
fn comparing_item_sequences() {
    let packed = "hello everyone";
    let spaced = "hello   everyone";
    let obscure = "hello you";

    assert_ne!(packed, spaced);
    assert!(packed.split_whitespace().eq(spaced.split_whitespace()));
    /* ' ' < 'e' */
    assert!(spaced < packed);
    /* everyone < you*/
    assert!(spaced.split_whitespace().lt(obscure.split_whitespace()));
}

#[test]
fn test_any_and_all() {
    let a = [1, 2, 3, 4, 5];
    let greater_than_three = |x: &i32| -> bool {*x > 3};

    assert!(a.iter().any(greater_than_three));
    assert!(!a.iter().all(greater_than_three));
}

/// position 从左往右查找元素在序列中第一次出现的位置，返回元素在序列中的索引
///
/// rposition 从右往左查找元素在序列中第一次出现的位置，返回元素在序列中的索引
/// rposition 要求 reversible iterator ，才可以从右端开始查找元素，
/// 还需要精确大小的迭代器，可以像位置一样分配索引。
#[test]
fn test_position_and_r_position() {
    let s = "eden";
    assert_eq!(s.chars().position(|c| c == 'e'), Some(0));
    assert_eq!(s.chars().position(|c| c == 's'), None);

    let b = b"eden";
    assert_eq!(b.iter().rposition(|&c| c == b'e'), Some(2));
}

/// fold(accumulator, fn)
///
/// accumulator 是一个累加器，fn 闭包重复作用于当前的累加器和 iterator 的下一个元素。
#[test]
fn test_fold_and_rfold() {
    let a = [1, 2, 3, 4, 5];
    assert_eq!(a.iter().fold(0, |n, _| n + 1), 5);
    assert_eq!(a.iter().fold(0, |n, i| n + i), 15);
    assert_eq!(a.iter().fold(1, |n, i| n * i), 120);

    assert_eq!(a.iter().fold(   i32::MIN,
                                |a, &i| -> i32 {
                                        if a >= i { a }
                                        else { i }
                                    }), 5);

    let s = ["hello", "darkness", "my", "old", "friend"];
    let s = s.iter().rfold(String::new(), |string, w| string + w + " ");
    println!("{}", s);
}

#[test]
fn test_try_fold_and_try_rfold() {
    let a = ["1", "2", "3", "4", "5"];
    let result = a.clone().iter()
        .try_fold(0, |sum, i| -> Result<i32, Box<dyn Error>> {
            Ok(sum + i32::from_str(&i.trim()).unwrap())
        });

    assert_eq!(result.unwrap(), 15);

    let result = a.clone().iter()
        .try_rfold(0, |sum, i| -> Result<i32, Box<dyn Error>> {
            Ok(sum + i32::from_str(&i.trim()).unwrap())
        });
    assert_eq!(result.unwrap(), 15);
}

#[test]
fn test_nth_and_nth_back() {
    let a = [1, 2, 3, 4, 5];
    let mut iter = a.iter();
    assert_eq!(iter.nth(2), Some(&3));
    assert_eq!(iter.nth_back(0), Some(&5));
    assert_eq!(iter.nth(7), None);
}

#[test]
fn test_last() {
    let mut v: Vec<i32> = Vec::new();
    assert_eq!(v.last(), None);

    v = vec![1, 2, 3, 4, 5];
    assert_eq!(v.last(), Some(&5));
}

#[test]
fn test_find_rfind_find_map() {
    #[derive(Debug, PartialEq)]
    struct Person {name: String, age: u32}
    let mut employees = vec![
        Person {name: "An".to_string(), age: 22},
        Person {name: "Wong".to_string(), age: 28},
    ];

    let search_n =
        |p: &&Person| p.name.contains('n');

    let mut n = employees.iter().find(search_n);
    assert_eq!(n, Some(&Person {name: "An".to_string(), age: 22}));
    n = employees.iter().rfind( search_n);
    assert_eq!(n, Some(&Person {name: "Wong".to_string(), age: 28}));

    employees = vec![];
    n = employees.iter().find(search_n);
    assert_eq!(n, None);

    let generations = |p: &Person| -> Option<(String, String)> {
        let name = p.name.clone();
        if p.age > 30 {
            return Some(("Elder".to_string(), name))
        }
        None
    };

    employees.push(Person {name: "Ma".to_string(), age: 44});
    let s = employees.iter()
        .find_map(generations);
    assert_eq!(s, Some(("Elder".to_string(), "Ma".to_string())));
}

/// extend 方法将一个 iterable 的所有元素添加到现有的集合中
#[test]
fn test_extend() {
    let mut v: Vec<i32> = (0..4).map(|i| 1 << i).collect();
    v.extend(&[10, 20, 30]);
    assert_eq!(v, &[1, 2, 4, 8, 10, 20, 30]);
}

/// partition 将一个 iterator 切分成两个集合
#[test]
fn test_partition() {
    #[derive(PartialEq, Debug)]
    struct Vehicle {name: String, channel: String}
    let vehicles = vec![
        Vehicle {name: "Steamship".to_string(), channel: "Waterway".to_string()},
        Vehicle {name: "Train".to_string(),     channel: "Rail".to_string()},
        Vehicle {name: "Bicycle".to_string(),   channel: "Land".to_string()},
        Vehicle {name: "Car".to_string(),       channel: "Land".to_string()},
    ];

    let (water, non_water): (Vec<&Vehicle>, Vec<&Vehicle>) =
        vehicles.iter()
            .partition(|&vehicle| vehicle.channel.eq(&"Waterway".to_string()));

    let print_vehicles = |v: Vec<&Vehicle>| println!("{:?}", v);

    print_vehicles(water);
    print_vehicles(non_water);
}

/// for_each 对 iterator 进行内部迭代，
/// 如果迭代的过程中可能会出错，可以使用 try_foreach
#[test]
fn test_for_each_and_try_for_each() {
    ["Nationalism", "Democracy", "The people's livelihood"].iter()
        .zip(1..4)
        .map(|(doctrine, i)| format!("{}. {}", i, doctrine))
        .for_each(|s| println!("{}", s));
}