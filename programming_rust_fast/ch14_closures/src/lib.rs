mod capture_variables;

#[derive(Debug)]
struct City {
    name: String,
    population: i64
}

fn cities_population_descending(city: &City) -> i64 {
    -city.population
}

/// 通过比较 City 中的 population 字段的值对 city 进行排序，
/// cities_population_descending 是一个辅助函数
/// 但是更简单的方式是使用闭包 -> 匿名的函数表达式
fn sort_cities(cities: &mut Vec<City>) {
    cities.sort_by_key(cities_population_descending);
}

#[test]
fn test_sort_cities() {
    let mut cities = vec![
        City {name: "Wu you".to_string(), population: 100_000},
        City {name: "Peng lai".to_string(), population: 999},
        City {name: "Tao hua yuan".to_string(), population: 10_000},
    ];

    sort_cities(&mut cities);
    println!("cities => {:?}", cities);
}

#[test]
fn test_sort_cities_with_closure() {
    let mut cities = vec![
        City {name: "Wu you".to_string(), population: 100_000},
        City {name: "Peng lai".to_string(), population: 999},
        City {name: "Tao hua yuan".to_string(), population: 10_000},
    ];

    /*
    |city| -city.population 是一个闭包，
    接收一个 city 实参，
    返回 -city.population
    Rust 从使用闭包的地方推断实参的类型和返回值的类型

     */
    cities.sort_by_key(|city| -city.population);

    for city in cities {
        println!("{} ", city.name);
    }
}