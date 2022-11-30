use std::collections::BinaryHeap;

/// 二进制堆是一个松散组织的元素集合，
/// 堆中最大的元素会冒泡出现在堆中的最顶端
///
/// 二进制堆最常见的用法是作为一个任务队列
/// 定义任务结构，实现 Ord trait 对于任务的优先级进行排序
#[test]
fn test_binary_heap() {
    let mut heap = BinaryHeap::from(vec![1, 2, 3, 4, 5, 6, -1]);
    heap.push(-10);
    // pop 弹出最大元素,如果没有则返回 None
    assert_eq!(heap.pop(), Some(6));
    // peek 返回最大元素的引用,如果没有则返回 None
    assert_eq!(heap.peek(), Some(&5));
    assert_eq!(heap.peek(), Some(&5));
}