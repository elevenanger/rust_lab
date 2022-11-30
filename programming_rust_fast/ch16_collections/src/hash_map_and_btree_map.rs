use std::collections::{BTreeMap, HashMap};

/// HashMap 和 BtreeMap 的方法基本一致
/// 只是 entry 在内存中的布局不同
#[test]
fn test_hash_map() {
    let mut map = HashMap::new();
    map.insert(1, 'a');
    map.insert(2, 'b');
    map.insert(3, 'c');

    assert!(!map.is_empty());
    assert!(map.contains_key(&1));
    assert_eq!(map.get(&1), Some(&'a'));

    let mut map2 = HashMap::new();
    map2.insert(4, 'd');
    map2.insert(5, 'e');

    map.extend(map2.iter());
    assert_eq!(map.len(), 5);
    assert_eq!(map.remove(&5), Some('e'));
    assert_eq!(map.remove_entry(&4), Some((4, 'd')));
    // retain 保留符合闭包的 entry
    map.retain(|&k, _| k > 1);
    assert_eq!(map.len(), 2);
    map.clear();
    assert!(map.is_empty());
}

#[test]
fn test_entry() {
    struct Food {name: String, price: f32}
    impl Food {

        fn with_name_and_price(name: &str, price: f32) -> Food {
            Food {name: name.to_string(), price}
        }

        fn new() -> Food {
            Food {name: "free".to_string(), price: 0.0}
        }

    }

    let mut food_map = BTreeMap::new();
    food_map.insert("banana", Food::with_name_and_price("banana", 4.5));
    food_map.insert("apple", Food::with_name_and_price("apple", 6.5));
    food_map.entry("apple").and_modify(|cur| {
        cur.name = "fuji apple".to_string();
        cur.price = 8.8
    });
    assert_eq!(food_map.get("apple").unwrap().price, 8.8);

    let food = food_map.entry("free")
                            .or_insert_with(Food::new);
    assert_eq!(food.name, "free".to_string());
    assert_eq!(food_map.len(), 3);
}