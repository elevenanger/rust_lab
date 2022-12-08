/// 原始指针是没有约束的指针，
/// 有两种原始指针
/// - *mut T 是可以修改引用对象的指针
/// - *const T 是只能读取引用对象的指针
#[test]
fn test_raw_pointers() {
    let mut x = 10;
    let ptr_x = &mut x as *mut i32;

    // 通过引用创建原始指针
    // 使用 * 操作符来解引用
    let y = Box::new(20);
    let ptr_y = &*y as *const i32;

    unsafe {
        *ptr_x += *ptr_y
    }

    assert_eq!(x, 30)
}

/// 原始指针可以为 null
/// 和 C 语言中的空指针一样
fn option_to_raw<T>(opt: Option<&T>) -> *const T {
    match opt {
        None => std::ptr::null(),
        Some(r) => r as *const T
    }
}

#[test]
fn test_option_to_raw() {
    assert!(!option_to_raw(Some(&("pea", "pod"))).is_null());
    assert_eq!(option_to_raw::<i32>(None), std::ptr::null())
}

/// offset_from() 返回两个指针之间的距离
#[test]
fn test_offset_from() {
    let v = vec![1, 2, 3];
    let first: *const i32 = &v[0];
    let second: *const i32 = &v[1];
    assert_eq!(unsafe { first.offset_from(second)}, -1)
}

mod ref_with_flag {
    use std::marker::PhantomData;
    use std::mem::align_of;

    pub struct RefWithFlag<'a, T> {
        ptr_and_bit: usize,
        behaves_like: PhantomData<&'a T>
    }

    impl<'a, T: 'a>RefWithFlag<'a, T> {
        pub fn new(ptr: &'a T, flag: bool) -> RefWithFlag<T> {
            assert!(align_of::<T>() % 2 == 0);
            RefWithFlag {
                ptr_and_bit: ptr as *const T as usize | flag as usize,
                behaves_like: PhantomData
            }
        }

        pub fn get_ref(&self) -> &'a T {
            unsafe {
                let ptr = (self.ptr_and_bit & !1) as *const T;
                &*ptr
            }
        }

        pub fn get_flag(&self) -> bool {
            self.ptr_and_bit & 1 != 0
        }
    }

    #[test]
    fn test_ref_with_flag() {
        let v = vec![10 , 20, 30];
        let flagged = RefWithFlag::new(&v, true);
        assert_eq!(flagged.get_ref()[1], 20);
        assert_eq!(flagged.get_flag(), true)
    }
}
