struct Appellation {
    name: String,
    nickname: Vec<String>
}

/// Rust 会在值丢弃的时候自动处理清理的工作，
/// 但是，可以自己实现 Rust Drop 值的方式，
/// 实现 Drop trait 即可。
///
/// 实现了 Drop trait ，
/// Rust 会在丢弃变量的值之前调用 drop 方法，
/// 这种隐式的调用方式是调用 drop 方法的唯一方式。
impl Drop for Appellation {
    fn drop(&mut self) {
        println!("Dropping {} ...", self.name);
        if !self.nickname.is_empty() {
            print!("{} is dropped.", self.nickname.join(", "));
        }
        println!();
    }
}

#[test]
fn test_appellation_drop() {
    let mut a  = Appellation {
        name: "Sun Wukong".to_string(),
        nickname: vec![
            "Mei hou wang".to_string(),
            "Da shi xiong".to_string()
        ]
    };

    println!("Before Dropping.");
    a = Appellation {name: "Wang".to_string(), nickname: vec![]};
    println!("End of block.");
}