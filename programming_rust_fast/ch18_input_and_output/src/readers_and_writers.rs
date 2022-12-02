use std::fs::{File};
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Read, stdout, Write, Error};

const DEFAULT_BUF_SIZE: usize = 8 * 1024;

type TestResult = Result<(), Error>;

/// Readers 是程序可以从中读取字节的值
///
/// Writers 是程序可以往里面写入字节的值
///
/// 将任意实现了Read trait 的数据拷贝到实现了 Write trait 的值
pub fn copy<R: ?Sized, W: ?Sized>(reader: &mut R, writer: &mut W) -> io::Result<u64>
where R: Read, W: Write {
    let mut buf = [0; DEFAULT_BUF_SIZE];
    let mut written = 0;
    loop {
        let len = match reader.read(&mut buf) {
            Ok(0) => return Ok(written),
            Ok(len) => len,
            Err(ref e) if e.kind() == ErrorKind::Interrupted => continue,
            Err(e) => return Err(e),
        };
        writer.write_all(&buf[..len])?;
        written += len as u64
    }
}

#[test]
fn test_read_and_write() {
    // 将数据写入 stdout
    let nums: Vec<u8> = (0..=100).collect();
    write!(stdout(), "{:?}", nums).unwrap();

    // file::open(filename) 打开文件，产生一个 reader
    let mut cargo_info = File::open("Cargo.lock").expect("文件打开失败");
    copy(&mut cargo_info, &mut stdout()).expect("");
}

#[test]
fn test_read() {
    // 设置 buffer size
    const BUFFER_SIZE: usize = 128;
    // 创建 buffer
    let mut buffer = [0; BUFFER_SIZE];
    // 创建 reader 从文件中读入数据
    let mut file_reader = File::open("Cargo.lock").expect("文件打开失败");
    // 写入数据字节数计数，初始化为 0
    let mut written_size = 0;
    // 创建 writer
    let mut writer = stdout();
    loop {
        // read() 方法将数据读入 buffer 中, 返回这次读取的字节数
        let len = match file_reader.read(&mut buffer) {
            // 为 0 表示已经读取完所有的数据，返回
            Ok(0) => return,
            // len 表示读入的字节数，返回本次读入的字节数
            Ok(len) => len,
            // 如果是 Interrupt 异常，则忽略该异常继续进行读取
            Err(ref e)  if e.kind() == ErrorKind::Interrupted => continue,
            // 其它异常则结束程序
            Err(_) => return
        };
        // 将 buffer 中的数据写入 writer
        writer.write_all(&buffer[0..len]).expect("数据写入失败");
        // 累加写入的字节数
        written_size += len as u64;
        println!("written size => {}", written_size);
    }
}

/// read_to_end 将 reader 中剩余的字符追加到一个 Vec<u8> 中
#[test]
fn test_read_to_end() -> TestResult {
    let mut reader = File::open("Cargo.lock")?;
    let mut vec:Vec<u8> = Vec::new();
    let size = reader.read_to_end(&mut vec)?;
    write!(stdout(), "{}\nsize => {}", String::from_utf8(vec).expect(""), size)?;
    Ok(())
}

/// read_to_string 将 reader 中剩余的字符读入一个 String 中
#[test]
fn test_read_to_string() -> TestResult {
    let mut s = String::new();
    let mut reader = File::open("Cargo.lock")?;
    let size = reader.read_to_string(&mut s)?;
    write!(stdout(), "{}\nsize => {}", s, size)?;
    Ok(())
}

/// read_exact 读取 buffer 大小的数据，
/// 如果数据在读入 buffer.len() 之前读取完了数据，
/// 返回 ErrorKind::UnexpectedEof error
#[test]
fn test_read_exact() -> TestResult {
    let mut reader = File::open("Cargo.lock")?;
    let mut buffer = [0; 10];
    reader.read_exact(&mut buffer)?;
    let mut writer = stdout();
    writer.write_all(&buffer)?;
    writer.flush()?;
    Ok(())
}

/// reader.read_bytes() 返回一个输入流的字节 iterator ，
/// 逐个字节读取输入流的数据，
/// 性能较差
#[test]
fn test_reader_bytes() -> TestResult {
    File::open("Cargo.lock")?
        .bytes()
            .for_each(|b| print!("{} ", b.unwrap()));
    Ok(())
}

/// reader1.chain(reader2) 将两个 reader 的数据连接起来
#[test]
fn test_reader_chain() -> TestResult {
    let reader1 = File::open("Cargo.lock")?;
    let reader2 = File::open("Cargo.toml")?;
    let mut buf = [0; 256];
    reader1.chain(reader2).read(&mut buf)?;
    stdout().write_all(&buf)?;
    stdout().flush()?;
    Ok(())
}

/// take(n) 返回一个新的 reader
/// 只读入前 n 字节的数据
#[test]
fn test_reader_take() -> TestResult {
    let mut reader = File::open("Cargo.toml")?.take(20);
    let mut s = String::new();
    reader.read_to_string(&mut s)?;
    stdout().write_all(&s.as_bytes())?;
    stdout().flush()?;
    Ok(())
}

/// buffered reader 同时实现了 Read 和 BufRead trait,
/// readline(&mut line) 方法 将一行的数据追加到 line => String 中,
/// 换行符也包含在 line 中
#[test]
fn test_buffered_read() -> TestResult {
    let mut reader = BufReader::new(File::open("Cargo.toml")?);
    let mut s = String::new();
    let mut line = 0;
    loop {
        let len = match reader.read_line(&mut s) {
            Ok(0) => return Ok(()),
            Ok(len) => {
                line += 1;
                len
            },
            Err(e) => return Err(e)
        };
        println!("line {} => {} bytes : {}", line, len, s);
        s.clear()
    }
}

/// lines() 返回一个输入行数据的 iterator ,
/// 这个方法常用于处理文本输入
#[test]
fn test_reader_lines() -> TestResult {
    let reader = BufReader::new(File::open("Cargo.toml")?);
    let mut line_no = 0;
    reader.lines().for_each(|line| {
        line_no += 1;
        println!("line {} => {}", line_no, line.unwrap())
    });
    Ok(())
}

/// read_until(b, buf) 从 input 读取到指定字符，
/// 将读取到的数据写入 buf 中
#[test]
fn test_read_until() -> TestResult {
    let mut buffer: Vec<u8> = Vec::new();
    let mut reader = BufReader::new(File::open("Cargo.toml")?);

    reader.read_until(b'\n',&mut buffer)?;
    stdout().write_all(buffer.as_slice())?;
    stdout().flush()?;
    Ok(())
}

/// split() 对读入的数据进行拆分
#[test]
fn test_read_split() -> TestResult {
    let reader = BufReader::new(File::open("Cargo.toml")?);
    reader.split(b'a')
        .map(Result::unwrap)
        .for_each(|part| println!("{}", String::from_utf8_lossy(part.as_slice()))
    );
    Ok(())
}

fn grep<R>(target: &str, reader: R) -> TestResult
where R: BufRead {
    for line_result in reader.lines() {
        let line = line_result?;
        if line.contains(target) {
            println!("{}", line)
        }
    }
    Ok(())
}

#[test]
fn test_grep() -> TestResult {
    let reader = File::open("Cargo.toml")?;
    grep("[", BufReader::new(reader))
}

