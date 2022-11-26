use std::collections::{BTreeMap, HashMap};
use std::iter::repeat;
use std::iter::Peekable;
use std::str::Chars;

#[test]
fn test_filter_and_map() {
    let song =   "We are the world\n\
                        Yellow\n\
                        That's why you go away\n";

    let song_vec: Vec<String> = song.lines()
        .filter(|s| s.len() > 10)
        .map(str::trim)
        .map(str::to_uppercase)
        .collect();

    println!("{:?}", song_vec);
}

/// filter_map 适配器，
/// 组合 filter 和 map，
/// 使用闭包将元素转换成另一个元素，
/// 或者当转换失败的时候丢弃元素
#[test]
fn filter_map_test() {
    use std::str::FromStr;
    let text = "2 some you 3 da 5 fan 10";
    let nums: Vec<i32> = text.split_whitespace()
        .filter_map(|s| i32::from_str(s).ok())
        .collect();

    assert_eq!(nums, [2, 3, 5, 10]);
}

/// flatmap 将闭包产生的序列连接到一起
#[test]
fn test_flat_map() {
    let mut city_foods = HashMap::new();
    city_foods.insert("Changsha",
                        vec!["Chou Tofu", "Crayfish", "Sugar and oil"]);
    city_foods.insert("Guang Zhou",
                        vec!["White cut chicken", "Shrimp dumpling"]);

    let foods: Vec<&str> = city_foods
        .into_iter()
        .flat_map(|city| city.1 )
        .collect();

    println!("{:?}", foods);
}

///flatten 连接多个迭代器的元素
#[test]
fn test_flatten() {
    let mut myth_hero = BTreeMap::new();
    myth_hero.insert("China", vec!["Hao Tian", "Fu Xi", "Nv Wa"]);
    myth_hero.insert("Greece", vec!["Zeus", "Apollo", "Ares"]);
    myth_hero.insert("India", vec!["Brahma", "Asura", "Vishnu"]);

    let all_hero:Vec<_> = myth_hero.values()
        .flatten()
        .cloned()
        .collect();

    println!("{:?}", all_hero);

    /*
    因为 Option 实现了 IntoIterator
    可以使用 flatten 对于 Option 中的 Some(v) 进行迭代，
    因为 None 对于迭代无意义，
    Some(v) 产生 v 的值，
    最终就获得 v 的值
    对于 Result 类型也有同样的效果
     */
    let options =
        vec![None, Some("one"), Some("two"), None, Some("four")];
    let nums: Vec<_> = options
        .into_iter()
        .flatten()
        .collect();
    println!("{:?}", nums);

}

#[test]
fn test_take_and_take_while() {
    /* take(n) 获取前 n 个元素 */
    let nums = vec![1, 2, 3, 4, -1, 9, 10];
    let first_three:Vec<_> = nums.clone().into_iter().take(3).collect();
    assert_eq!(first_three, [1, 2, 3]);

    /* take_while(fn)
    获取直到不满足闭包条件的元素
    */
    let until_negative: Vec<_> = nums
        .into_iter()
        .take_while(|x| *x > 0)
        .collect();
    assert_eq!(until_negative, [1, 2, 3, 4]);
}

#[test]
fn test_skip_and_skip_while() {
    let cities = vec!["Chang Sha", "Wu Han", "Dong Guan", "Guang Zhou"];

    let guangdong_city: Vec<_> =
        cities.clone().into_iter().skip(2).collect();
    assert_eq!(guangdong_city, vec!["Dong Guan", "Guang Zhou"]);


    let skip_chang: Vec<_> = cities.into_iter()
        .skip_while(|city| city.contains("Chang"))
        .collect();
    assert_eq!(skip_chang, vec!["Wu Han", "Dong Guan", "Guang Zhou"]);
}

#[test]
fn test_peekable() {
    let parse_number =
    |token: &mut Peekable<Chars>| -> u32 {
        let mut n = 0;
        loop {
            match token.peek() {
                Some(r) if r.is_digit(10) =>
                    n = n * 10 + r.to_digit(10).unwrap(),
                _ => return n
            }
            token.next();
        }
    };

    let mut chars = "12345,54321".chars().peekable();

    assert_eq!(parse_number(&mut chars), 12345);
    assert_eq!(chars.next(), Some(','));
    assert_eq!(parse_number(&mut chars), 54321);
    assert_eq!(chars.next(), None);
}

/// fuse 熔断适配器：
/// 一旦 Iterator 返回 None，
/// 继续调用 next 一般会返回 None，
/// 但是并不总是这样，
/// fuse 适配器适配一个 Iterator 一旦它第一次返回 None ，
/// 后续都会继续返回 None,
/// fuse 一般适用于那些不确定应用范围的泛型代码。
#[test]
fn test_fuse() {
    struct Flaky (bool);

    impl Iterator for Flaky {
        type Item = &'static str;

        fn next(&mut self) -> Option<Self::Item> {
            if self.0 {
                self.0 = false;
                Some("Till the end.")
            } else {
                self.0 = true;
                None
            }
        }
    }

    let mut flaky = Flaky(true);
    assert_eq!(flaky.next(), Some("Till the end."));
    assert_eq!(flaky.next(), None);
    assert_eq!(flaky.next(), Some("Till the end."));
    assert_eq!(flaky.next(), None);

    let mut fuse_flaky = Flaky(true).fuse();
    assert_eq!(fuse_flaky.next(), Some("Till the end."));
    assert_eq!(fuse_flaky.next(), None);
    assert_eq!(fuse_flaky.next(), None);
    assert_eq!(fuse_flaky.next(), None);
}

/// 一些 Iterator 可以从序列的两端遍历元素，
/// 这些 Iterator 实现了 DoubleEndedIterator trait
/// 对于这种 Iterator 可以使用 rev 适配器进行翻转。
#[test]
fn test_reversible_iterator_and_rev() {
    let elapse = vec!["past", "now", "future"];
    let mut elapse_iter = elapse.iter();
    assert_eq!(elapse_iter.next(),      Some(&"past"));
    assert_eq!(elapse_iter.next_back(), Some(&"future"));
    assert_eq!(elapse_iter.next(),      Some(&"now"));

    let mut elapse_iter = elapse.iter().rev();
    assert_eq!(elapse_iter.next(),      Some(&"future"));
    assert_eq!(elapse_iter.next(),      Some(&"now"));
    assert_eq!(elapse_iter.next_back(), Some(&"past"));
}

#[test]
fn test_inspect() {
    let upper_case: String = "anger".chars()
        .inspect(|c| println!("before : {:?}", c))
        .flat_map(|c| c.to_uppercase())
        .inspect(|c| println!("after : {:?}", c))
        .collect();

    assert_eq!(upper_case, "ANGER".to_string());
}

/// chain 适配器将一个 Iterator 追加至另一个 Iterator
/// 如果两个 Iterator 都是 reversible ，
/// 则 chained iterator 也是 reversible
#[test]
fn test_chain() {
    let single_digits = [1, 2, 3];
    let tens_digit = [10, 20, 30];
    let digits: Vec<_> =
        single_digits.into_iter().chain(tens_digit).collect();

    assert_eq!(&digits, &vec![1, 2, 3, 10, 20, 30]);

    assert_eq!(digits.into_iter().rev().collect::<Vec<i32>>(),
                vec![30, 20, 10, 3, 2, 1]);
}

/// enumerate 产生 (index, element) 序列
#[test]
fn test_enumerate() {
    let chars = vec!['A', 'B', 'C'];
    let pairs:Vec<_> = chars.into_iter().enumerate().collect();
    let mut iter = pairs.into_iter();
    assert_eq!(iter.next(), Some((0, 'A')));
    assert_eq!(iter.next(), Some((1, 'B')));
    assert_eq!(iter.next(), Some((2, 'C')));

    let chars_slice = &['D', 'E', 'F'];
    let pair:Vec<_> =
        chars_slice[1..]
            .into_iter().enumerate().collect();

    println!("{:?}", pair);

}

/// zip 将两个 iterator 组合成键值对 iterator
#[test]
fn test_zip() {
    let nums = vec![0, 1, 2].into_iter();
    let chars = vec!['a', 'b', 'c'].into_iter();

    let v: Vec<_> = nums.zip(chars).collect();
    assert_eq!(v,   vec![(0, 'a'), (1, 'b'), (2, 'c')]);

    let warning: Vec<_> =
        (1..=3).zip(repeat("Fishing is prohibited")).collect();

    let warning: Vec<_> = warning
        .into_iter()
        .map(|s| format!("{}.{}", s.0, s.1))
        .inspect(|s| println!("{}", s))
        .collect();

    assert_eq!(warning.len(), 3);
}

#[test]
fn test_by_ref() {
    let mut v = vec![1, 2, 3, 4].into_iter();
    let ten_v: Vec<_> = v.by_ref().map(|x| x * 10).collect();
    assert_eq!(ten_v, vec![10, 20 ,30 ,40]);
}

#[test]
fn test_cloned_and_copied() {
    let a = &[1, 2, 3];
    let ac:Vec<_> = a.into_iter().cloned()
        .map(|i| i * 2)
        .collect();

    assert_eq!(a, &[1, 2, 3]);
    assert_eq!(ac, vec![2, 4, 6]);

    let acp: Vec<_> = a.into_iter().copied()
        .map(|i| i * 3)
        .collect();
    assert_eq!(a, &[1, 2, 3]);
    assert_eq!(acp, vec![3, 6 , 9]);
}

/// cycle 不断循环一个 iterator
#[test]
fn test_cycle() {
    let again = ["Heavenly God Dao",
                            "Human Dao",
                            "Asura Dao",
                            "Hell Dao",
                            "Hungry Ghost Dao",
                            "Animal Dao",
    ];

    let over_again: Vec<_> = again.iter()
        .cycle()
        .take(12)
        .inspect(|dao| println!("{}", dao))
        .collect();

    assert_eq!(over_again.len(), 12);
}