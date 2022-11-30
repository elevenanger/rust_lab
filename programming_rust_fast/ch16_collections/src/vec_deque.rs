use std::collections::VecDeque;

#[test]
fn test_vec_deque() {
    let mut vd = VecDeque::new();
    vd.push_front(1);
    vd.push_front(0);
    vd.push_back(2);

    assert_eq!(vd, [0, 1, 2]);
    assert_eq!(vd.front(), Some(&0));
    assert_eq!(vd.back(), Some(&2));

    assert_eq!(vd.pop_front(), Some(0));
    assert_eq!(vd.pop_back(), Some(2));

    let mut v: Vec<i32> = Vec::from(vd);
    assert_eq!(v, [1]);
    v.push(2);

    let vd = VecDeque::from(v);
    assert_eq!(vd, [1, 2]);
}