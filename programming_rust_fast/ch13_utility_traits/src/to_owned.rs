struct S<T>
where T: ?Sized {
    v: T
}

/// ToOwned trait ：
/// 将引用转换成拥有的值
impl ToOwned for S<i32> {
    type Owned = S<i32>;

    fn to_owned(&self) -> Self::Owned {
        S {v: self.v}
    }
}

#[test]
fn test_to_owned_s() {
    let s = &S {v: 31};
    let i = s.to_owned();
    assert_eq!(i.v, 31);
}