mod personal_trait;
mod self_in_traits;
mod subtraits;
pub mod type_associated_functions;

use std::fs::File;
use std::io::Write;

/// 定义 trait ：
/// trait TraitName
/// 即可定义一个 trait ，
pub trait Profiling {
    /// 定义 trait 的函数，实现 trait 的类型都需要实现 trait 中所有的函数。
    fn process<T: Write>(&self, write: &mut T);

    fn info(&self) -> String;

    /// 默认实现:
    /// 在 trait 中已经实现的方法，
    /// 不需要做 impl 代码块中进行实现，
    /// 实现 trait 的类型可以直接使用该方法。
    fn print_profile(&self) {
        println!("{}", self.info());
    }
}

#[derive(Clone)]
pub struct Person {
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
        write.write_all(self.info().as_ref()).expect("数据写入失败");
    }

    fn info(&self) -> String {
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

pub trait CreatureSet {
    /// 大多是面向对象的语言中，
    /// 接口不能有 static 方法或者构造函数，
    /// 但是 trait 可以包括 类型关联函数，
    /// 类比于面向对象语言中的 static 方法。
    ///
    /// 所有实现 CreatureSet trait 的类型，都需要实现其中的函数，
    /// 这个函数和其它的类型关联函数一样，
    /// 可以直接使用调用类型关联函数的方式进行调用。
    /// ```
    /// let an = Person::from_name_and_age("An", 18);
    /// ```
    /// trait 对象不支持类型关联函数，
    /// 如果想要使用 &dyn CreatureSet trait 对象，
    /// 必须改变 trait ，
    /// 添加约束 where Self: Sized 给那些没有接收引用 self 作为参数的函数。
    ///
    /// 这个约束告诉 Rust ， trait 对象可以不需要支持这个函数。
    fn from_name_and_age(name: &str, age: u8) -> Self
        where Self: Sized;

    /// trait 对象还是可以正常使用有 self 参数的函数
    fn grow_up(&mut self, age: u8);

}