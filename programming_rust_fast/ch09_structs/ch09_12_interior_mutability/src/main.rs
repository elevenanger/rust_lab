use std::cell::{Cell, RefCell};
use std::fs::File;
use std::io::Write;

fn main() { }

/// 需要对一个不可变的值的部分数据做修改，
/// 这种行为称为 内部可变性。
/// Rust 提供几种机制来进行这种操作,
/// Cell<T> 和 RefCell<T> ,
/// Cell<T> 是一种包括一个私有的 T 类型的值的结构。
/// Cell 的特殊之处在于，
/// 可以通过它的 get 和 set 方法访问它所有的 T 类型的值，而不需要 mut 权限。
///
/// RefCell<T> 支持借用 T 类型值的引用，
/// 和其它的 mut reference 一样，同时只能有一个地方借用了同一个 mut reference 。
struct Person {
    name: String,
    age: Cell<u32>,
    /* 为了记录日志，需要一个可变的 File
        cell 返回的是它的值的拷贝，不能调用共享值的 mut 方法
        RefCell 可以借用值的引用调用它的 mut 方法 */
    log_file: RefCell<File>,
}

impl Person {
    fn with_name_and_age(name: &str, age: u32) -> Person {
        Person {
            name: name.to_string(),
            /* Cell::new(value) 创建一个 Cell，将 value 值转移给它 */
            age: Cell::new(age),
            /* RefCell::new(value) 创建一个 RefCell 将 value move 给它 */
            log_file: RefCell::new(File::create(format!("{}.log", name)).unwrap())
        }
    }

    /// 只需要 &self 即可修改 Cell 中 u32 类型的值
    fn grow_up(&self) {
        /* cell.set(value) 将 value 存储到 cell 中，丢弃之前的值
            这个方法不需要 &mut self
            只需要接收 &self 作为参数 */
        self.age.set(self.age.get() + 1);

        let grow_log = format!("{} is {} years old.", self.name, self.age.get());
        self.write_log(&grow_log);
    }

    fn write_log(&self, message: &str) {
        /* refcell.borrow_mut() 返回 refcell 中的值的 mut reference */
        let mut file = self.log_file.borrow_mut();
        writeln!(file, "{}", message).unwrap();
    }
}

#[test]
fn test_person_grow() {
    let an = Person::with_name_and_age("an", 18);
    an.grow_up();
    /* call.get() 拷贝并返回 cell 中的值
        .get() 方法会返回 cell 的值的一份副本
        所以它仅支持实现了 Copy trait 的类型 */
    assert_eq!(an.age.get(), 19);

    for _ in 0..10 {
        an.grow_up();
    }
}