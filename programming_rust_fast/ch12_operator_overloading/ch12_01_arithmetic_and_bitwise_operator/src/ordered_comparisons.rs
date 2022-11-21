use std::cmp::{Ordering, PartialOrd, Reverse};

/// 排序比较：
/// PartialOrd
#[derive(Debug, PartialEq)]
struct Interval<T> {
    higher: T,
    lower: T
}

impl<T> PartialOrd<Interval<T>> for Interval<T>
where T: PartialOrd {
    fn partial_cmp(&self, other: &Interval<T>) -> Option<Ordering> {
        if self == other {
            Some(Ordering::Equal)
        } else if self.lower > other.higher {
            Some(Ordering::Greater)
        } else if self.higher < other.lower {
            Some(Ordering::Less)
        } else {
            None
        }
    }

}

#[test]
fn test_complex_partial_ord() {
    let c1 = Interval {higher: 4, lower: 3};
    let c2 = Interval {higher: 6, lower: 5};
    let c3 = Interval {higher: -1, lower: -2};
    let c4 = Interval {higher: 4, lower: 3};
    let c5 = Interval {higher: 3, lower: 2};

    assert_eq!(c4, c1);
    assert_eq!(c1.partial_cmp(&c5), None);
    assert!(c1 < c2);
    assert!(c1 > c3);

    let mut intervals = vec![c1, c2, c3, c4];
    intervals.sort_by_key(|k| k.higher);
    println!("intervals => {:?}", intervals);

    intervals.sort_by_key(|k| Reverse(k.lower));
    println!("intervals => {:?}", intervals);
}