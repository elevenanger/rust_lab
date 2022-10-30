extern crate core;

use std::str::FromStr;
use num::Complex;
use image::{ColorType, ImageEncoder};
use image::codecs::png::PngEncoder;
use std::fs::File;
use std::env;


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 5 {
        eprintln!("用法：{} FILE PIXEL UPEERLEFT LOWERRIGHT", args[0]);
        eprintln!("例子: {} mandel.png 1000*750 -1.20,0.35 -1,0.20", args[0]);
        std::process::exit(1);
    }

    let bounds = parse_pair(&args[2], 'x')
                        .expect("解析图片错误");
    let upper_left = parse_complex(&args[3])
                        .expect("上左端点解析失败");
    let lower_right = parse_complex(&args[4])
                        .expect("下右端点解析失败 ");
    let mut pixels = vec![0; bounds.0 * bounds.1];

    render(&mut pixels, bounds, upper_left, lower_right);
}

/// <T:FromStr> 表示任意实现了 FromStr 的类型
/// Option<(T, T)> 返回值类型，是两个 T 类型值的元组 可能是 None 或者 some((v1, v2))
fn parse_pair<T:FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    /*
    在 s 中查找匹配 separator 字符
    整个 match 表达式的结果为 None 表示解析失败，没有匹配的数据
     */
    match s.find(separator) {
        None => None,
        Some(index) => {
            /*
             (T::from_str(&s[..index]), T::from_str(&s[index + 1..]))
             &s[..index] 和 &s[..index + 1..] 是字符串的切片，在分隔符之前和之后
             类型参数的 from_str 函数接收每一个参数将其转换成 T 类型值，产生元组结果
             (Ok(l), Ok(r)) => Some((l, r)) 这个模式匹配元组的两个元素都是 Result 的 Ok 变体的情况
             如果匹配成功 Some((l, r)) 是能够符合表达式的值，同时作为函数的返回值
             _ => None 通配符表达式匹配任何内容，并忽略它的值
             */
            match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
                (Ok(l), Ok(r)) => Some((l, r)),
                _ => None
            }
        }
    }
}

fn parse_complex(s: &str) -> Option<Complex<f64>> {
    match parse_pair(s, ',') {
        Some((re, im)) => Some(Complex {re, im}),
        None => None
    }
}

fn pixel_to_point(bounds:(usize, usize),
                    pixel:(usize, usize),
                    upper_left:Complex<f64>,
                    lower_right:Complex<f64>)
    -> Complex<f64> {
    let (width, height) = (lower_right.re - upper_left.re,
                                        upper_left.im - lower_right.im);

    Complex {
        re: upper_left.re + pixel.0 as f64 * width / bounds.0 as f64,
        im: upper_left.im - pixel.1 as f64 * height / bounds.1 as f64
    }
}

fn render(pixels: &mut [u8],
            bounds: (usize, usize),
            upper_left: Complex<f64>,
            lower_right: Complex<f64>) {
    assert!(pixels.len() == bounds.0 * bounds.1);

    for row in 0..bounds.1 {
        for column in 0..bounds.0 {
            let point = pixel_to_point(bounds, (column, row),
                                                        upper_left, lower_right);

            pixels[row * bounds.0 + column] =
                match escape_time(point, 255) {
                    None => 0,
                    Some(count) => 255 - count as u8
                };
        }
    }
}

fn write_image(filename:&str, pixels: &[u8], bounds: (usize, usize))
    -> Result<(), std::io::Error> {
    /*
    ? 操作符可以不用每次显式地写出错误逻辑
    如果 create 函数失败 ？ 操作符从方法返回，传递错误信息
    否则 output 持有成功打开的文件
     */
    let output = File::create(filename)?;

    let encoder = PngEncoder::new(output);
    encoder.write_image(&pixels,
                        bounds.0 as u32, bounds.1 as u32,
                        ColorType::L8)
                            .expect_err("绘图异常");
    Ok(())
}

fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> {
    let mut z = Complex {re: 0.0, im: 0.0};
    for i in  0..limit {
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
        z = z * z + c;
    }
    None
}

#[test]
fn test_pixel_to_point () {
    assert_eq!(pixel_to_point((100, 200), (25, 175),
                                Complex {re: -1.0, im: 1.0},
                                Complex {re: 1.0, im: -1.0}),
                Complex {re : -0.5, im: -0.75});
}

#[test]
fn test_parse_complex() {
    assert_eq!(parse_complex("1.0,2.0"),
                Some(Complex{re: 1.0, im: 2.0}));
    assert_eq!(parse_complex(",1.2"),
                None);
}

#[test]
fn test_parse_pair() {
    assert_eq!(parse_pair::<i32>("",        ','), None);
    assert_eq!(parse_pair::<i32>("10",      ','), None);
    assert_eq!(parse_pair::<i32>(",10",     ','), None);
    assert_eq!(parse_pair::<f64>("0.5 1.0", ' '), Some((0.5, 1.0)));
}