use std::env;
use text_colorizer::*;
use std::fs;
use regex::Regex;
/// #[derive(Debug)] 告诉编译器生成额外的代码
/// 在使用 println! 等方法的时候可以使用 {:?} 的形式格式化 Arguments 结构
#[derive(Debug)]
struct Arguments {
    target: String,
    replacement: String,
    filename: String,
    output: String,
}

fn main() {
    let args = parse_args();
    println!("{:?}", args);

    // 将文件中的文本读入字符串
    let data = match fs::read_to_string(&args.filename) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{} 文件读取失败 '{}':{:?}",
                        "Error:".red().bold(), args.filename, e);
            std::process::exit(1);
        }
    };

    // 将字符串按照指定的正则表达式进行替换
    let replaced_data = match replace(&args.target, &args.replacement, &data) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{} 字符串替换失败 : {:?}",
                        "Error:".red().bold(), e);
            std::process::exit(1);
        }
    };

    // 将 data 中的数据写入文件
    match fs::write(&args.output, &replaced_data) {
        Ok(_) => {},
        Err(e) => {
            eprintln!("{} 文件写入错误 '{}' {:?}",
                        "Error".red().bold(), args.filename, e);
            std::process::exit(1);
        }
    }
}

fn print_usage() {
    eprintln!("{} - 将文本中出现的字符串替换为指定的字符串",
                "quickreplace:".green());
    eprintln!("用法：quickreplace <目标字符串><替换字符串><输入><输出>");
}

fn parse_args() -> Arguments {

    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() != 4 {
        print_usage();
        eprintln!("{} 参数数量错误，预期 4 个参数，实际 {} 个参数",
                    "Error: ".red().bold(), args.len());
        std::process::exit(1);
    }

    // 将命令行参数转换为 Arguments 结构兑现
    Arguments {
        target: args[0].clone(),
        replacement: args[1].clone(),
        filename: args[2].clone(),
        output: args[3].clone()
    }
}

/// 使用 regex 替换文本中的字符串为目标字符串
fn replace(target: &str, replace: &str, text: &str) ->
    Result<String, regex::Error> {
    // 正则表达式
    let regex = Regex::new(target)?;
    // 将 text 中符合正则表达式的字符串替换为目标字符串
    Ok(regex.replace_all(text, replace).to_string())
}