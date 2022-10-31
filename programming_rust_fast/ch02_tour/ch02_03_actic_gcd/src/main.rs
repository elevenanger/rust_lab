
/*
使用 use actix_web::{ ... }
花括号中的函数可以直接在当前文件中使用
 */
use actix_web::{web, App, HttpResponse, HttpServer};
/*
serde 提供处理表单数据的工具
 */
use serde::Deserialize;

/*
定义 Rust 结构类型
接收表单数据
 */
#[derive(Deserialize)]
struct GcdParameter {
    n:u64,
    m:u64,
}

fn main() {
    /*
    || {...}
    Rust 闭包表达式
    闭包是一个可以像函数一样调用的值
    启动服务之后
    每次请求进来调用闭包
    使用一个新的 APP 的值根据路由来处理请求
     */
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(get_index))
            /* 添加路由处理 post 请求 */
            .route("/gcd", web::post().to(post_gcd))
            .route("/anger", web::get().to(anger_welcome))
    });

    println!("启动服务 http://localhost:3000");
    server.bind("127.0.0.1:3000")
        .expect("端口绑定失败")
        .run()
        .expect("服务启动失败");
}

fn get_index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html;charset=UTF-8")
        .body(
            /*
            r#"..."#
            因为 body 的内容会包含很多的双引号
            使用这种 Rust 原生字符串格式
            中间所有的字符都会当成一个字符串来处理
             */
            r#"
                <title>GCD Calculator</title>
                <form action="/gcd" method="post">
                <input type="text" name="n"/>
                <input type="text" name="m"/>
                <button type="submit">Compute GCD</button>
                </form>
            "#
        )
}

fn anger_welcome() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("application/json")
        .body("message:welcome")
}

/*
post_gcd(form: web::Form<GcdParameter>)
post_gcd 函数接收一个实参 form
参数类型为 Form<GcdParameter>
 */
fn post_gcd(form: web::Form<GcdParameter>) -> HttpResponse {
    if form.n == 0 || form.m == 0 {
        return HttpResponse::BadRequest()
            .content_type("text/html;charset=UTF-8")
            .body("计算 0 的最大公约是是没有意义的");
    }
    let response =
        format!("{} 和 {} 的最大公约数是 => <b>{}</b>\n",
                form.n, form.m, gcd(form.m, form.n));

    HttpResponse::Ok()
        .content_type("text/html;charset=UTF-8")
        .body(response)
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}
