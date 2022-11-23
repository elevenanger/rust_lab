use std::thread;
use crate::{City};

/// 闭包可以使用封闭函数的数据：
/// is_ascend 就是一个封闭函数的数据，
/// 可以在闭包中使用，
/// 闭包捕获了这个数据
///
/// 在这个例子中，当 Rust 创建了这个闭包，
/// 就会借用 is_ascend 的引用，
/// 因为闭包包含了 is_ascend 的引用，
/// 它的生命周期不会超过 is_ascend ，
/// 因此这个闭包仅在排序时使用。
fn sort_by_population(cities: &mut Vec<City>, is_ascend: bool) {
    cities.sort_by_key(|city|
        {
            if is_ascend {
                city.population
            } else {
                -city.population
            }
        }
    )
}


/// key_fn 包含了一个对 is_ascend 的引用，
/// 但是 Rust 无法保证 is_ascend 的使用安全，
/// 后面的 cities 也存在同样的问题，
/// 使用 move 关键字告诉 Rust 将 cities 和 is_ascend move 到闭包中。
///
/// Rust 提供两种方法为闭包从封闭的作用域中获取数据：
/// move 和 借用。
///
/// ```
/// let key_fn = move |city: &City| ->
///     i64 {if is_ascend { city.population} else { -city.population }};
/// ```
///
fn sort_with_thread(mut cities: Vec<City>, is_ascend: bool)
    -> thread::JoinHandle<Vec<City>> {
    let key_fn = move |city: &City| ->
        i64 {if is_ascend { city.population} else { -city.population }};

    thread::spawn(move || {
        cities.sort_by_key(key_fn);
        cities
    })
}

/// 函数类型值：
/// 可以和其他类型值一样使用函数，
/// 可以将函数存储在一个变量中，
/// 使用常规的 Rust 语法计算函数的值，
/// struct 可以有函数字段，
/// 函数都是 fn 类型，
/// fn 的值是一个指向函数机器字的指针。
///
/// 闭包和函数的类型不同，
/// Fn trait 类型既可以接收函数参数也可以接收闭包参数。
///
/// 闭包是可调用的，但是不是 fn 类型，
/// 事实上，每个闭包都有它自己的类型，
/// 因为闭包可能会包含数据 -> 借用或者捕获的值，
/// 这些值可以是任意类型的组合，
/// 所以编译器会为每个闭包生成一个独一无二的类型，
/// 它的大小足以储存它所拥有的数据值，
/// 没有两个类型完全一样的闭包，
/// 但是每个闭包都实现了 Fn trait，
/// 因为闭包没有固定的类型，
/// 所以使用闭包的代码一般是泛型的。
fn count_cities_population<F>(cities: &Vec<City>, test_fn: F) -> i64
where F: Fn(&City) -> bool {
    let mut count = 0;
    for city in cities {
        if test_fn(city) {
            count += city.population
        }
    }
    count
}

fn population_over_one_thousands(city: &City) -> bool {
    match city.population {
        0..=1000 => false,
        _ => true
    }
}

#[test]
fn test_population_count() {
    let cities = City::new_cities();

    let cities_population =
        count_cities_population(&cities, population_over_one_thousands);
    println!("{}", cities_population);

    let cities_population =
        count_cities_population(&cities,
                            |city| city.population > 10_000);
    println!("{}", cities_population);
}

#[test]
fn test_sort_by_population() {
    let mut cities = City::new_cities();
    sort_by_population(&mut cities, false);
    println!("{:?}", cities);

    sort_by_population(&mut cities, true);
    println!("{:?}", cities);
}

#[test]
fn test_sort_with_thread() {
    let cities = City::new_cities();
    println!("{:?}", sort_with_thread(cities, false).join());
}