use fern_sim::{Fern, run_simulation};

/// 库和命令行程序可以共存于同一个源码库。
///
/// 将命令行程序的代码放在 srs/bin 目录
/// 使用 use 为 fern_sim 库中部分需要使用的 item 创建本地别名。
/// 使用 cargo run --bin efern 命令运行 efern 程序
fn main() {
    let mut fern = Fern {
        size: 1.0,
        growth_rate: 0.02
    };

    run_simulation(&mut fern, 100);
    println!("final size => {}", fern.size);
}