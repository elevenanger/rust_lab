use std::ops::{Mul};
use std::sync::Arc;
use std::thread;

#[test]
fn test_spawn() {
    // 创建一个空 vec 用于存放线程
    let mut threads = vec![];
    // thread::spawn 方法创建线程，
    // 方法中的闭包是每个线程要执行的任务
    // push 线程到 vec 中
    for i in 0..20 {
        threads.push(thread::spawn(move|| -> Vec<i32> {
            let start = i * 100;
            let end = i * 100 + 100;
            print_numbers(start, end)
        }))
    }

    // join 方法等待所有线程的任务处理完毕
    let nums: Vec<_> = threads.into_iter()
        .flat_map(|t| t.join().unwrap())
        .collect();

    println!("{}", nums.len());
}

fn print_numbers(mut start: i32, end: i32) -> Vec<i32> {
    let mut v = Vec::new();
    while start < end {
        println!("Thread => {:?}:{:?} count {}",
                    thread::current().id(),
                    thread::current().name(), start);
        start += 1;
        v.push(start);
    }
    v
}

/// 使用 Arc 包装数据在多个线程之间共享数据
#[test]
fn test_share_immutable_date_between_threads() {
    let multiplier = Arc::new(100);
    let mut work_lists = Vec::new();
    for i in 0..20 {
        let m = multiplier.clone();
        work_lists.push(thread::spawn(move || -> Vec<i32> {
            print_numbers(m.mul(i), m.mul(i + 1))
        }))
    }

    let nums: Vec<i32> = work_lists.into_iter()
                            .flat_map(|work_list| work_list.join().unwrap())
                            .collect();
    println!("nums len => {}", nums.len())
}

#[test]
fn test_rayon() {
    use rayon::prelude::*;
    let nums: Vec<_> = (0..2000).collect();
    let s = nums.par_iter()
        .map(|num| format!("thread => {:?} {:?} num => {}\n",
                                            thread::current().name(),
                                            thread::current().id(), num))
        .reduce_with(|r1, r2| r1 + &r2)
        .unwrap();
    println!("{}", s);
}