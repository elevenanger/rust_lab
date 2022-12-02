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
            let mut v = Vec::new();
            let mut start = i * 100;
            while start < i * 100 + 100 {
                println!("Thread => {:?}:{:?} count {}",
                            thread::current().id(),
                            thread::current().name(), start);
                start += 1;
                v.push(start);
            }
            v
        }))
    }

    // join 方法等待所有线程的任务处理完毕
    let nums: Vec<_> = threads.into_iter()
        .flat_map(|t| t.join().unwrap())
        .collect();

    println!("{}", nums.len());
}