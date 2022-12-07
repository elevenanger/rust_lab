use crate::unsafe_blocks::my_ascii::Ascii;

#[test]
fn test_unsafe_block() {
    let s = b"unsafe";
    // 在 unsafe 块中使用 unsafe 方法
    unsafe {
        println!("s => {}", String::from_utf8_unchecked(s.to_vec()));
    }

}

mod my_ascii {
    #[derive(Debug, PartialEq, Eq)]
    pub struct Ascii (Vec<u8>);

    impl Ascii {
        pub fn from_bytes(bytes: Vec<u8>) -> Result<Ascii, NotAsciiError> {
            if bytes.iter().any(|&byte| !byte.is_ascii()) {
                return Err(NotAsciiError(bytes));
            }
            Ok(Ascii(bytes))
        }

        /// 定义 unsafe 函数
        pub unsafe fn from_bytes_unchecked(bytes: Vec<u8>) -> Ascii {
            Ascii(bytes)
        }
    }

    #[derive(Debug, Eq, PartialEq)]
    pub struct NotAsciiError(pub Vec<u8>);

    impl From<Ascii> for String {
        fn from(ascii: Ascii) -> Self {
            // 在函数中使用 unsafe 代码块
            unsafe {
                String::from_utf8_unchecked(ascii.0)
            }
        }
    }
}

#[test]
fn test_my_ascii() {
    let bytes: Vec<u8> = b"this is my Ascii".to_vec();

    let my_ascii = Ascii::from_bytes(bytes).unwrap();

    let s = String::from(my_ascii);

    println!("{}", s);
}

#[test]
fn test_my_ascii_from_bytes_unchecked() {
    let bytes = vec![0xff, 0xbf, 0xaf];

    let ascii = unsafe {
        Ascii::from_bytes_unchecked(bytes)
    };

    println!("{}", String::from(ascii));
}