/// 将模块标记为 #[cfg(test)] 作为测试模块
#[cfg(test)]
mod tests {
    fn print_divide(x: i32, y: i32) {
        println!("result of {} / {} => {}", x, y, x / y);
    }

    /// 测试异常用例，
    /// 使用 should_panic 注解，
    #[test]
    #[should_panic]
    fn test_print_divide() {
        print_divide(100, 0);
    }
}