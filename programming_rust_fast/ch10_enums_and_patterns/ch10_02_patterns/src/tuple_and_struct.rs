mod tuple_and_struc {
    use crate::{Person, Sex};

    /// 模式匹配元组：
    pub fn describe_point(x: i32, y: i32) -> &'static str {
        use std::cmp::Ordering::*;
        match (x.cmp(&0), y.cmp(&0)) {
            (Equal, Equal) => "坐标原点",
            (Equal, _) => "y 轴",
            (_, Equal) => "x 轴",
            (Greater, Greater) => "第一象限",
            (Less, Greater) => "第二象限",
            (Less, Less) => "第三象限",
            _ => "第四象限"
        }
    }

    #[test]
    fn test_describe_point() {
        println!("{}", describe_point(0, 0));
        println!("{}", describe_point(0, 1));
        println!("{}", describe_point(1, 0));
        println!("{}", describe_point(1, 1));
        println!("{}", describe_point(-1, 1));
        println!("{}", describe_point(-1, -1));
        println!("{}", describe_point(1, -1));
    }



    impl Person {

        /// 结构的模式匹配：
        /// ```
        /// // .. 表示忽略其它的字段
        /// Person {name: name, sex: Sex::Female, ..}
        /// ```
        fn title(&self) -> String {
            match self {
                Person {name, sex: Sex::Female, ..} =>
                    format!("Mrs {}", name),
                Person {name, sex: Sex::Male, ..} =>
                    format!("Mr {}", name),
            }
        }

    }

    #[test]
    fn test_person_title() {
        let an = Person {name: "An".to_string(), sex: Sex::Female, age: 12};
        println!("{}, {} years old", an.title(), an.age);

        let wong = Person {name: "Wong".to_string(), sex: Sex::Male, age: 20};
        println!("{}, {} years old", wong.title(), wong.age);
    }
}