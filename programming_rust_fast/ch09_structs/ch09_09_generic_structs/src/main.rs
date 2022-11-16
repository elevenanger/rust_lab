fn main() {
}

/// 泛型结构：
/// Rust 的结构可以为泛型，
/// 可以定义一个模板结构插入任何类型。
struct Queue<T> {
    older: Vec<T>,
    younger: Vec<T>
}

/// <> 三角括号接收类型参数，
/// 作为泛型 impl ，需要在 impl 后面加上 <T>,
/// 表示对于任意类型，这里定义的与其相关联的类型函数。
impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {older: Vec::new(), younger: Vec::new()}
    }

    pub fn push(&mut self, t:T) {
        self.younger.push(t);
    }

    pub fn is_empty(&self) -> bool {
        self.older.is_empty() && self.younger.is_empty()
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        if self.older.is_empty() {
            use std::mem::swap;
            swap(&mut self.older, &mut self.younger);
            self.older.reverse();
        }
        self.older.pop()
    }
}

/// 为某个特定的类型定义关联函数：
/// impl Queue<i32> 中的关联函数,
/// 只有在类型为 i32 的时候才能调用。
impl Queue<i32> {
    pub fn sum(&self) -> i32 {
        let mut sum = 0;

        for younger_num in &self.younger {
            sum += younger_num;
        }

        for older_num in &self.older {
            sum += older_num;
        }

        sum
    }
}

#[test]
fn test_generic_queue() {
    let mut i32_queue = Queue::<i32>::new();
    i32_queue.push(1);
    i32_queue.push(2);
    i32_queue.push(3);
    assert_eq!(i32_queue.sum(), 6);
    assert_eq!(i32_queue.pop(), Some(1));
    assert_eq!(i32_queue.pop(), Some(2));
    assert_eq!(i32_queue.pop(), Some(3));
    assert!(i32_queue.is_empty());

    let mut string_queue = Queue::<String>::new();
    string_queue.push("we".to_string());
    string_queue.push("are".to_string());
    string_queue.push("one".to_string());
    while !string_queue.is_empty() {
        print!("{} ", string_queue.pop().unwrap());
    }
    println!();

    let mut str_queue = Queue::new();
    str_queue.push("type");
    str_queue.push("infer");
    while !str_queue.is_empty() {
        print!("{} ", str_queue.pop().unwrap());
    }
}