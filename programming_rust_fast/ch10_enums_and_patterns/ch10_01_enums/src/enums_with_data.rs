pub mod enums_with_data {

    /// 单元变体：
    /// 只有枚举值，没有其它数据的变体。
    #[derive(Debug)]
    pub enum TimeUnit {
        Day,
        Month,
        Year
    }

    impl TimeUnit {
        pub fn plural(&self) -> String {
            match self {
                TimeUnit::Day => "day".to_string(),
                TimeUnit::Month => "month".to_string(),
                TimeUnit::Year => "year".to_string()
            }
        }
    }

    /// 元组变体：
    /// RoughUnit 中的 Past(TimeUnit, i32) 和 Future(TimeUnit, i32) 变体，
    /// 类似于元组结构，
    /// 称之为元组变体。
    /// 元组变体的构造方式：
    /// ```
    /// let one_day_ago = RoughTime::Past(TimeUnit::Day, 1);
    /// ```
    #[derive(Debug)]
    pub enum RoughTime {
        Past(TimeUnit, u32),
        Now,
        Future(TimeUnit, u32)
    }

    #[test]
    fn test_tuple_variation() {
        let now = RoughTime::Now;
        let one_day_ago = RoughTime::Past(TimeUnit::Day, 1);
        let two_months_ago = RoughTime::Past(TimeUnit::Month, 2);
        let three_years_later = RoughTime::Future(TimeUnit::Year, 3);

        println!("{:?}", now);
        println!("{:?}", one_day_ago);
        println!("{:?}", two_months_ago);
        println!("{:?}", three_years_later);
    }

    /// 命名字段结构变体：
    /// 枚举类型为命名字段结构,
    /// 枚举类型中所有构造器以及字段都与枚举拥有相同的可见性。
    #[derive(Debug)]
    enum TwoDimensionalShape {
        Circle {radius: f32},
        Square {side_length: f32},
    }

    #[test]
    fn test_named_field_variation() {
        let circle = TwoDimensionalShape::Circle {radius: 5.5};
        let square = TwoDimensionalShape::Square {side_length: 3.3};

        println!("{:?}", circle);
        println!("{:?}", square);
    }
}