use::num::Complex;
use ::std::str::FromStr;
use std::env::args;

fn main() {
    let mut numbers = Vec::new();

    for arg in args().skip(1) {
        numbers.push(f64::from_str(&arg));
    }

    if numbers.len() == 0 {
        eprintln!("用法 mandelbrot 数字1 数字2 ", );
        std::process::exit(1);
    }

    let re = numbers[0].as_ref().expect("re 转换失败");
    let im = numbers[1].as_ref().expect("im 转换失败");
    let limit  = 2000;

    println!("re = {} im = {}", *re, *im);

    let result = escape_time(Complex {re: *re, im: *im}, limit);

    println!("逃逸时间 ：{}", result.expect("参数转换异常"));
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
