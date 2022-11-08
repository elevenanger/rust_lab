fn main() {
    break_expression("Anger");

    continue_keywords("hello you");

    let mut sentences = Vec::new();
    sentences.push("you are the one".to_string());
    sentences.push("quiet space".to_string());
    sentences.push("darkness".to_string());

    labeled_break(&sentences);

    labeled_continue(&sentences);

    labeled_value_expression(&sentences);
}

/// break 表达式：
/// [break] 表达式退出 [loop] 循环，
/// 在 [Rust] 中，
/// [break] 语句仅仅在 [loop] 循环中使用，
/// 在 [loop] 表达式内，可以给 [break] 一个表达式，
/// 表达式的值会成为 [loop] 表达式的值,
/// 每个 [break] 表达式产生相同类型的值，
/// 这将成为 [loop] 表达式本身的值类型。
///
/// ```
/// let answer = loop {
///     if let Some(ch) = chars.next() {
///         if ch == 'A' {
///             break "Has A";
///         }
///     } else {
///         break "Not A";
///     }
/// };
/// ```
fn break_expression(sentence: &str) {
    let mut chars = sentence.chars();
    let answer = loop {
        if let Some(ch) = chars.next() {
            if ch == 'A' {
                break "Has A";
            }
        } else {
            break "Not A";
        }
    };
    println!("{}", answer);
}

/// continue 关键字：
/// 在 for 循环中，
/// continue 关键字继续前进到集合中下一个值，
/// 如果没有下一个值，则退出循环，
/// 同理，在 while 循环中，continue 重新检查循环条件，
/// 如果条件为 false，循环结束。
fn continue_keywords(sentence: &str) {
    let chars = sentence.chars();
    for char in chars {
        if char == ' ' {
            continue
        }
        print!("{}", char.to_uppercase());
    }
    println!()
}

/// 循环标签：
/// 循环可以打上标签，
/// 在循环内 break 和 continue 可以和标签一起使用，
/// break 或者 continue 作用于标签所指的循环。
/// ```
/// 'search_out:
/// for sentence in sentences {
///     for char in sentence.chars() {
///         if char == 'q' {
///            println!("quit outer loop.");
///             break 'search_out
///         }
///         print!("{}", char);
///     }
///     println!()
/// }
/// ```
/// break 'search_out 退出当前的 for sentence in sentences 循环。
/// continue 关键字同理。
fn labeled_break(sentences: &Vec<String>) {
    println!("labeled break");
    'search_out:
    for sentence in sentences {
        for char in sentence.chars() {
            if char == 'q' {
               println!("quit outer loop.");
                break 'search_out
            }
            print!("{}", char);
        }
        println!()
    }
}

fn labeled_continue(sentences: &Vec<String>) {
    println!("labeled continue");
    'search_out:
    for sentence in sentences {
        for char in sentence.chars() {
            if char == 'q' {
               println!("quit outer loop.");
                continue 'search_out
            }
            print!("{}", char);
        }
        println!()
    }
}

/// break 表达式可以同时使用标签和值表达式：
fn labeled_value_expression(sentences: &Vec<String>) {
    let q_sen= 'outer: loop {
        for sentence in sentences {
            for char in sentence.chars() {
                if char == 'q' {
                    break 'outer sentence
                }
            }
        }
    };
    println!("sentence with q => {}", q_sen);
}