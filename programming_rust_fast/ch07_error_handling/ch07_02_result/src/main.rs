use std::error::Error;
use std::fmt;
use std::fmt::Formatter;
use std::io::{Write, stderr};

fn main() {
    result();
}

/// Result 类型：Result<T, E>
/// 说明可能出现故障
fn result() {

    propagating_errors().expect("error");

    catch_errors();

    custom_error_type();
}

/// 捕获错误：
/// - 最全面的处理错误的方式是使用 match 表达式：
/// ```
/// let r = "1das".parse::<u32>();
/// match r {
///     Ok(num) => println!("parse result => {}", num),
///     Err(err) => println!("parse failure {}", err)
/// }
/// ```
/// 这种处理方式类似于其它语言的 try/catch 语句，
/// 当需要直接处理错误，而不需要将错误传递给调用方的时候使用这种方式。
fn catch_errors() {
    let r = &mut "1das".parse::<u32>();
    let r2 = &mut "1".parse::<u32>();

    match r {
        Ok(num) => println!("parse result => {}", num),
        Err(err) => print_err(err)
    }

    /* 返回一个 bool 值表示 result 的结果是成功还是失败 */
    if r.is_err() {
        /* .err() Option<E> 如果存在错误，返回错误值 */
        println!("err info : {:?}", r.as_ref().err());
    }
    if r2.is_ok() {
        /* .ok() 返回结果，Option<E> 如果是成功的结果 返回 Some(suc_val)
        不然返回 None ，丢弃错误值 */
        println!("parse result : {:?}", r2.as_ref().ok());
    }
    /* .unwrap_or(bak_val) 如果成功则返回成功的结果值
        如果失败则返回 bak_val 丢弃 err 值 */
    println!("r result => {}", r.as_ref().unwrap_or(&0));
    /* .unwrap() 如果结果是成功的返回成功的结果，如果结果是失败的，引起 panic */
    println!("r2 result => {}", r2.as_ref().unwrap());
    /* 和 .unwrap() 一样的方式，但是可以在引起 panic 的时候打印一条错误信息
        .as_ref() 将 Result<T, E> 转换为 Result<&T, &E>
        .as_mut() 将 Result<T, E> 转换为 Result<&mut T, &mut E>
        这两个方法可以很方便的借用 result 而使 result 的值完好无损 */
    println!("r2 result => {}", r2.as_mut().expect("parse error"));
}

fn print_err(mut err: &dyn Error) {
    let _ = writeln!(stderr(), "error : {}", err);
    while let Some(source) = err.source() {
        let _ = writeln!(stderr(), "caused by : {}", source);
        err = source;
    }
}

type GenericError = Box<dyn Error + Send + Sync + 'static>;
type GenericResult<T> = Result<T, GenericError>;

/// 传递 error：
/// 如果错误发生了，但是需要调用者进行错误处理，需要将错误传向上传递给调用栈，
/// Rust 使用 ? 运算符来传递错误，
/// 可以在任何产生 Result 的表达式使用 ?
///
fn propagating_errors() -> GenericResult<i32> {
    let num = "2".parse::<i32>()?;
    println!("num = {}", num);
    Ok(num)
}

fn custom_error_type() {
    let an = build_personal_file("an", 18);

    match an {
        Ok(per) => println!("{} {}", per.name, per.age),
        Err(err) => println!("{:?}", err)
    }
}

#[derive(Debug, Clone)]
pub struct PersonError {
    pub message: String,
}

impl fmt::Display for PersonError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.message)
    }
}

impl Error for PersonError { }

struct Person {name: String, age: u8}

fn build_personal_file(name: &str, age: u8) -> Result<Person, PersonError> {
    if age > 200 {
        Err(PersonError {message: "error age".to_string()})
    }
    else {
        Ok(Person {name: name.to_string(), age})
    }
}