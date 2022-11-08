fn main() {
    if_let(100);
}

/// if let 表达式：
/// ```
/// if let pattern = expr {
///     block1
/// } else {
///     block2
/// }
/// ```
/// - 给定的 [expr] 满足 [pattern] 则执行 [block1], 否则执行 [block2]。
/// - [if let] 表达式能够很方便地从 [Option] 或者 [Result] 中提取数据
/// - [if let] 是只有一个 [pattern] 的 [match] 表达式的简写：
/// ```
/// match expr {
///     pattern => {block1}
///     _       => {block2}
/// }
/// ```
fn if_let(score: u8) {
    if let Some(str) = check_score(score) {
        println!("{} score.", str);
    } else {
        println!("Bad score");
    }
}

fn check_score(score: u8) -> Option<String> {
    if score > 0 && score <= 100 {
        Some("Right".to_string())
    } else {
        None
    }
}