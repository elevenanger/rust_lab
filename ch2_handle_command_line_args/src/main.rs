/*
    use 关键字引入其它的库文件
    引入标准库 trait 的 FromStr 
 */
use std::str::FromStr;
/*
    env 标准库
    提供一些与执行环境交互的函数和类型
    包括 args 函数
    提供访问命令行参数的方法
 */
use std::env;

fn main() {
    /*
        声明一个 numbers 变量
        初始化为一个空的 Vector 对象
     */
    let mut numbers = Vec::new();
    
    /**
        args() 函数返回一个迭代器
        迭代器的第一条数据始终为程序名
        跳过第一条数据，遍历剩余的数据
     */
    for arg in env::args().skip(1) {
        /*
            使用 Vec push 函数添加元素
         */
        numbers.push(
            /**
                使用 u64::from_str() 方法将 string 类型参数转换成 u64 类型
                from_str() 函数返回一个 Result 值
                Result 值表示转换是否成功
                Result 的值有两个变体
                1、Ok(v) 表明转换是成功的，v 是产生的值
                2、Err(e) 表示转换失败，e 是错误信息表示函数可能会失败的原因
                Rust 没有异常
                所有的错误使用 Result 或者 panic 处理
             */
            u64::from_str(&arg)
            /*
                如果 Result 的值是 err(e)
                expect 打印 e 的描述信息并退出程序
                如果结果是 Ok(v) 则返回 v 本身
             */
            .expect("error parsing argument"));   
    }

    if numbers.len() == 0 {
        /*
            如果没有元素
            说明没有命令行参数
            将错误输出到标注输出
            退出进程
         */
        eprintln!("用法 gcd number ... ");
        std::process::exit(1);
    }

    let mut d = numbers[0];
    /*
        在这个例子中
        我们的代码仅仅操作简单的整数类型值占有特定大小的内存空间
        但是这里迭代的是一些 vector 
        它可以拥有任意大小的内存空间
        Rust 对于这种值的操作非常谨慎
        它希望程序员控制内存消耗
        确保程序员清楚的知道每个值的生存时间
        当值不再需要的时候立即释放内存空间
        所以在进行迭代的时候
        我们希望 vector 的所有权应该是数字类型
        我们仅仅在这个循环中借用它
        & 操作符 &numbers[1..] 表示从 vector 的第二个元素开始借用每个元素的引用地址
        m 借用后续的每一个元素
        *m 中的 * 操作符表示对 m 的解引用，产生它指向的值
        因为 numbers 对 vector 有所有权
        Rust 会在 numbers 超出 main 函数范围的时候自动释放它
        Rust 的所有权和借用的概念是内存管理和并发安全的关键
     */
    for m in  &numbers[1..] {
        d = gcd(d, *m);
    }

    println!("{:?} 最大公约数 {}", numbers, d);
}


fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}