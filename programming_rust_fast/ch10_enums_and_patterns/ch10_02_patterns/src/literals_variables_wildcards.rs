pub mod literals_variables_wildcards {

    /// match 表达式模式匹配字面量：
    /// ```
    /// // 0 1 两个模式表示匹配整数值
    /// 0 => {}
    /// 1 => {println!("one rabbit.")}
    /// // n 表示一个变量名，它可以匹配任意的值，并将匹配的值 move 为代码中的局部变量。
    /// n => {println!("{} rabbits.", n)}
    /// ```
    pub fn count_rabbits(count: u32) {
        match count {
            0 => {}
            1 => {println!("one rabbit.")}
            n => {println!("{} rabbits.", n)}
        }
    }


    #[test]
    fn test_count_rabbits() {
        count_rabbits(0);
        count_rabbits(1);
        count_rabbits(10);
    }
}