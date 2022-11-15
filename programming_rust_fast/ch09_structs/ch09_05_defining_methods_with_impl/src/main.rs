fn main() {
}

struct Queue {
    older: Vec<char>,
    younger: Vec<char>
}

/// impl 是一组 fn 定义的集合，
/// impl struct
/// 每个接收 self 作为参数的函数都是结构的方法。
/// impl 中定义的方法称为关联方法：
/// 它们与特定的结构类型相关联，
/// 与之相对的是自由函数，
/// 没有与任何结构相关联，
/// 不定义在 impl 中。
impl Queue {

    /// 关联方法的第一个参数必须是 self ，
    /// self 是关联的结构类型的值或者引用，
    /// self 参数可以省略类型。
    ///
    /// self 参数前面可以加上 & 和 mut 运算符：
    /// &self 表示对 self 的引用，
    /// &mut self 表示可变引用，
    /// self 表示其本身的值。
    pub fn push(&mut self, c: char) {
        self.younger.push(c);
    }

    pub fn pop(&mut self) -> Option<char> {
        if self.is_empty() {
            return None;
        }

        if self.older.is_empty() {
            use std::mem::swap;
            swap(&mut self.older, &mut self.younger);
            self.older.reverse()
        }

        self.older.pop()
    }

    pub fn is_empty(&self) -> bool {
        self.older.is_empty() && self.younger.is_empty()
    }

    /// 接收 self 作为参数，
    /// 将 younger 和 older 的值返回移走，
    /// 该函数调用后，self 将会变为未初始化的状态。
    pub fn split(self) -> (Vec<char>, Vec<char>) {
        (self.younger, self.older)
    }

}

#[test]
fn test_queue() {
    let mut queue = Queue {older: Vec::new(), younger: Vec::new()};

    queue.push('A');
    queue.push('N');

    assert_eq!(queue.pop(), Some('A'));
    assert_eq!(queue.pop(), Some('N'));
    assert_eq!(queue.pop(), None);

    assert!(queue.is_empty());

    queue.push('G');
    assert!(!queue.is_empty());
    queue.pop();

    queue.push('I');
    queue.push('&');
    queue.pop();
    queue.push('U');

    let (young, older) = queue.split();

    assert_eq!(young, vec!['U']);
    assert_eq!(older, vec!['&']);
}

