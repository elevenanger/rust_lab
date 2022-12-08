pub unsafe trait Zeroable {}

unsafe impl Zeroable for usize { }

fn zeroed_vector<T>(len: usize) -> Vec<T>
where T: Zeroable {
    let mut vec = Vec::with_capacity(len);
    unsafe {
        std::ptr::write_bytes(vec.as_mut_ptr(), 0, len);
        vec.set_len(len);
    }

    vec
}

#[test]
fn test_zeroed_vector() {
    let v: Vec<usize> = zeroed_vector(100_000);
    assert!(v.iter().all(|&x| x == 0) );
}