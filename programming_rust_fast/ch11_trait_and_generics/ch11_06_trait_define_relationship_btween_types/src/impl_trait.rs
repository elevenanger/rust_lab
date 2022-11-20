use std::iter;
use std::vec::IntoIter;

fn cyclical_zip(v: Vec<u8>, u: Vec<u8>) ->
    iter::Cycle<iter::Chain<IntoIter<u8>, IntoIter<u8>>> {
    v.into_iter().chain(u.into_iter()).cycle()
}

fn cyclical_zip2(v: Vec<u8>, u: Vec<u8>) -> Box<dyn Iterator<Item = u8>> {
    Box::new(v.into_iter().chain(u.into_iter()).cycle())
}

fn cyclical_zip3(v: Vec<u8>, u: Vec<u8>) -> impl Iterator<Item = u8> {
    v.into_iter().chain(u.into_iter()).cycle()
}

#[test]
fn test_cyclical_zip() {
    let u: Vec<u8> = vec![1, 2, 3];
    let v: Vec<u8> = vec![4, 5, 6];

    println!("{:?}", cyclical_zip(v, u));
}

#[test]
fn test_cyclical_zip2() {
    let u: Vec<u8> = vec![1, 2, 3];
    let v: Vec<u8> = vec![4, 5, 6];

    let mut iter = cyclical_zip2(v, u);

    for _ in 0..6{
        println!("{}", iter.next().unwrap());
    }
}

#[test]
fn test_cyclical_zip3() {
    let u: Vec<u8> = vec![1, 2, 3];
    let v: Vec<u8> = vec![4, 5, 6];

    let mut iter = cyclical_zip3(v, u);

    for _ in 0..6{
        println!("{}", iter.next().unwrap());
    }
}
