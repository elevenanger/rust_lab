mod reference {
    use std::mem::swap;
    use crate::{Person, Sex};

    /// ref 借用匹配的值的一部分：
    fn greet_person(p: Person) {
        match p {
            Person {ref name, ref sex, ..} => {
                match sex {
                    Sex::Male => println!("Hello, Mr {}.", name),
                    Sex::Female => println!("Hello, Mrs {}.", name)
                }
            }
        }
    }

    /// 使用 ref mut 借用可变引用：
    fn change_name(mut p: Person, new_name: &str) -> Person {
        match p {
            Person {ref mut name, ..} =>
                swap(name, &mut new_name.to_string())
        }
        p
    }

    /// & 匹配引用：
    fn is_ma(p: &Person) -> bool {
        match &p.name.as_str() {
            // 使用 | 匹配多个条件
            &"Ma" | &"ma" => true,
            _ => false
        }
    }

    fn young_person(person: &Person) {
        match person.age {
            /*
            x @ pattern
            匹配 pattern ，但是匹配成功之后，
            不需要为匹配的值的一部分创建变量，
            而是创建一个变量 x ,
            将整个值 copy 或者 move 给它。

            18 ..= 30
            范围表达式，
            match 当前只支持这种写法，
            表示一个封闭区间。
            */
            age @ 18 ..= 30 => println!("{} years old young people.", age),
            _ => println!("Not young.")
        }
    }

    #[test]
    fn test_greet_person() {
        greet_person(Person {name: "An".to_string(), sex: Sex::Female, age: 18 });
        greet_person(Person {name: "Ma".to_string(), sex: Sex::Male, age: 22});
    }

    #[test]
    fn test_change_name() {
        let an = Person {name: "An".to_string(), sex: Sex::Female, age: 18 };
        let ann = change_name(an, "Ann");
        println!("{:?}", &ann);
    }

    #[test]
    fn test_is_ma() {
        let ma = Person {name: "Ma".to_string(), sex: Sex::Male, age: 55};
        assert!(is_ma(&ma));

        let ma1 = Person {name: "ma".to_string(), sex: Sex::Male, age: 55};
        assert!(is_ma(&ma1));
    }

    #[test]
    fn test_young_person() {
        young_person(&Person {name: "An".to_string(), sex: Sex::Female, age: 18 });
        young_person(&Person {name: "Ma".to_string(), sex: Sex::Male, age: 55});
    }

}