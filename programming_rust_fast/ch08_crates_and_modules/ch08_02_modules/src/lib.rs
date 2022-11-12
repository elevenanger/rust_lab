mod spores;
pub mod plant_structures;

/// 将一个程序转换成库的步骤：
/// - 重命名 src/main.rs 为 lib.rs
/// - 将库中的 public 特性加上 pub 关键字
/// 不需要改变 Cargo.toml 文件，
/// Cargo 会查找源目录来推算需要构建的内容，
/// 当它看到 lib.rs 就会将其视为一个库。
///
/// src/libs 中的代码作为库的根模块，
/// 其它使用该库的的 crate 只能访问这个根模块。
pub struct Fern {
    // 将 struct 中的域设置为 pub，使得其可以在模块之外可见
    pub size: f64,
    pub growth_rate: f64,
}

impl Fern {
    pub fn grow(&mut self) {
        self.size *= 1.0 + self.growth_rate;
    }
}

pub fn run_simulation(fern: &mut Fern, days: usize) {
    for _ in 0..days {
        fern.grow();
    }
}