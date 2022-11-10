fn main() {
    panic();
}


/// panic:
/// panic 是一种不应该发生的错误，
/// 程序遇到其本身的 bug 的时候会发生 panic，
/// 常见的 panic ：
/// - 数组越界访问
/// - 整数除以 0
/// - 断言失败
/// - 对于是 Err 的结果调用 .expect()
/// Rust 在 panic 发生的时候可以展开堆栈或者终止程序，展开堆栈是默认选项。
/// panic unwind stack 典型的处理过程如下：
/// - 在终端打印错误信息
/// - 展开堆栈：
///   任何的临时值、局部变量或者函数使用到的实参都会被丢弃，
///   与它们创建的顺序相反。
///   丢弃值之后会进行清理，任何程序使用的 String Vec 会被释放
///   打开的文件会被关闭，等等。。。
///   用户定义的 drop() 方法会被调用。
///   当前函数清理完成之后将继续按照相同的方式清理它的调用方。以此类推。
/// - 最终，当前线程会退出，如果触发 panic 的是主线程，整个进程都会结束(使用非 0 exit code)
/// panic 是针对线程的，一个线程出现 panic ，其它线程仍然可以照常运行。
/// 使用 catch_unwind() 方法可以捕获栈展开，运行程序继续运行，
/// 但是这种方式只能处理 unwind 栈的 panic，并非所有的 panic 都按照这种方式处理。
/// ```
///  let a = std::panic::catch_unwind(|| {
///      println!("per shared amount : {}", pirate_share(1000,0))
///  });
/// ```
/// 异常终止(abort)：
/// unwind 是默认的 panic 处理方式，但是以下两种场景，Rust 不会尝试 unwind stack:
/// - 在 Rust 第一次 panic 之后的 .drop() 方法触发了第二个 panic ，
/// Rust 继续尝试清理，这被认为是致命的，Rust 会停止 unwind 并且终止整个进程。
/// - Rust 的 panic 行为是可以自定义的，在编译的时候使用 -C panic = abort
/// 程序运行时第一次 panic 就会立即终止整个进程（使用这个选项 Rust 不需要知道如何 unwind stack
///  这可以减少编译后代码的体积）
fn panic() {
    panic_catch_unwind();
}

fn panic_catch_unwind() {
    let a = std::panic::catch_unwind(|| {
        println!("per shared amount : {}", pirate_share(1000,0))
    });
    assert!(a.is_err());
}

fn pirate_share(total: u64, crew_size: u64) -> u64 {
    let half = total / 2;
    half / crew_size
}