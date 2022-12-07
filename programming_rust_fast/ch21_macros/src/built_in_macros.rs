

#[test]
fn test_file_macro() {
    println!("{}", file!())
}

#[test]
fn test_line_macro() {
    println!("{}", line!())
}

#[test]
fn test_column_macro() {
    println!("{}", column!())
}

#[test]
fn test_stringify_macro() {
    let s = stringify!(raw string);
    println!("{}", s)
}

#[test]
fn test_concat_macro() {
    let concat_s = concat!("you ", "are ", "right!");
    println!("{}", concat_s)
}

#[test]
fn test_cfg_macro() {
    println!("{}", cfg!(debug_assertions))
}

#[test]
fn test_env_macro() {
    //env 输出环境变量
    println!("{}", env!("PATH"))
}
