use std::fs::File;
use std::io::Write;

/// 定义 trait ：
/// trait TraitName
/// 即可定义一个 trait ，
trait Profiling {
    /// 定义 trait 的函数，实现 trait 的类型都需要实现 trait 中所有的函数。
    fn process<T: Write>(&self, write: &mut T);

    fn profile_info(&self) -> String;

    /// 默认实现:
    /// 在 trait 中已经实现的方法，
    /// 不需要做 impl 代码块中进行实现，
    /// 实现 trait 的类型可以直接使用该方法。
    fn print_profile(&self) {
        println!("{}", self.profile_info());
    }
}

struct Person {
    name: String,
    age: u8,
}

/// 实现 trait：
/// impl TraitName for TypeName
/// 实现 trait 需要实现 trait 中所有的方法，
/// trait 实现代码块中的所有的特性都需要是 trait 本身的特性，
/// 如果需要为某个 trait 方法添加一个辅助方法，
/// 则需要在类型的实现中进行添加。
impl Profiling for Person {
    fn process<T: Write>(&self, write: &mut T) {
        write.write_all(self.profile_info().as_ref()).expect("数据写入失败");
    }

    fn profile_info(&self) -> String {
        format!("name : {} , age : {}", self.name, self.age)
    }

}

#[test]
fn test_profiling_for_person() {
    let an = Person {name: "An".to_string(), age: 18};
    let file_name = format!("{}.txt", &an.name);
    let mut local_file = File::create(file_name).expect("文件创建失败");
    an.process(&mut local_file);

    an.print_profile();
}
