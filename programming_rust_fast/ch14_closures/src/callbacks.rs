use std::collections::HashMap;

struct Request {
    url: String,
    method: String,
    headers: HashMap<String, String>,
    body: Vec<u8>
}

struct Response {
    code: u32,
    headers: HashMap<String, String>,
    body: Vec<u8>
}

/// 因为每个闭包的类型都以独一无二的
/// 所以需要定义一个实现了 Fn trait object 类表示不同的回调函数
type BoxedCallback = Box<dyn Fn (&Request) -> Response>;

struct BasicRouter {
    routes: HashMap<String, BoxedCallback>
}

impl BasicRouter{
    fn new() -> BasicRouter {
        BasicRouter {routes: HashMap::new()}
    }

    /// callback 函数 trait object 的两个限制：
    /// Fn (&Request) -> Response 限定了函数的请求和返回值类型，
    /// 'static 限定了函数的生命周期，
    /// 如果没有 'static 限定，
    /// Box::new(callback) 会出错，
    /// 因为闭包的入参数 &Request ，
    /// 一个借用的引用，
    /// 如果这个引用的声明周期超过闭包的声明周期是不安全的，
    /// 声明为 'static 生命周期为静态生命周期，
    /// 使得闭包的声明周期包含变量的生命周期.
    fn add_route<C>(&mut self, url: &str, callback: C)
    where C: Fn (&Request) -> Response + 'static {
        self.routes.insert(url.to_string(), Box::new(callback));
    }

}

impl BasicRouter {
    fn handle_request(&self, request: &Request) -> Response {
        match self.routes.get(&request.url) {
            None => Response::file_not_found(),
            Some(callback) => callback(request)
        }
    }
}

impl Request {
    fn get_request() -> Request {
        Request {
            url: "/".to_string(),
            method: "Get".to_string(),
            headers: HashMap::new(),
            body: "".to_string().as_bytes().to_vec()
        }
    }

    fn print_info(&self) {
        println!("request:");
        println!(  "url     => {}\n\
                    method  => {}\n\
                    headers => {:?}\n\
                    body    => {}\n",
                    self.url, self.method, self.headers,
                    String::from_utf8_lossy(&self.body))
    }

    fn set_body(&mut self, body: &str) {
        self.body = body.as_bytes().to_vec();
    }

    fn set_url(&mut self, url: &str) {
        self.url = url.to_string();
    }
}

impl Response {
    fn success() -> Response {
        Response {
            code: 200,
            headers: HashMap::new(),
            body: Vec::new()
        }
    }

    fn file_not_found() -> Response {
        Response {
            code: 404,
            headers: HashMap::new(),
            body: "404".as_bytes().to_vec()
        }
    }

    fn print_info(&self) {
        println!("response:");
        println!(  "code    => {}\n\
                    headers => {:?}\n\
                    body    => {}\n",
        self.code, self.headers, String::from_utf8_lossy(&self.body));
    }

    fn set_body(&mut self, body: &str) {
        self.body = body.as_bytes().to_vec();
    }
}

#[test]
fn test_basic_route() {
    let mut route = BasicRouter::new();
    let greet = |req: &Request| {
        req.print_info();
        let mut response = Response::success();
        response.set_body(
            &format!("Hello {}.", String::from_utf8_lossy(&req.body)));
        response
    };

    route.add_route("/", greet);
    let mut req = Request::get_request();
    req.set_body("Ann");

    let response = route.handle_request(&req);
    response.print_info();

    req.set_url("/not");
    let response = route.handle_request(&req);
    assert_eq!(response.code, 404);
}

#[test]
fn test_function_pointer() {
    // 函数指针
    let fn_ptr: fn(i32) -> i32 = add_ten;
    let eleven = fn_ptr(1);
    assert_eq!(eleven, 11);

    // 闭包指针
    let closure_ptr: fn(i32) -> i32 = |x| x + 10;
    let eleven = closure_ptr(1);
    assert_eq!(eleven, 11);
}

fn add_ten(x: i32) -> i32 {
    10 + x
}

/// 使用函数指针，
/// 限定调用者只能传入非捕获的闭包，
/// 提升一些性能和灵活性
struct FnPointerRouter {
    routes: HashMap<String, fn(&Request) -> Response>
}

impl FnPointerRouter {
    fn new() -> FnPointerRouter {
        FnPointerRouter {
            routes: HashMap::new()
        }
    }

    fn add_route(&mut self, url: &str, callback: fn(&Request) -> Response) {
        self.routes.insert(url.to_string(), callback);
    }

    fn handle_request(&self, req: &Request) -> Response {
        match self.routes.get(&req.url) {
            Some(callback) => callback(req),
            None => Response::file_not_found()
        }
    }
}

#[test]
fn test_fn_router() {
    let mut router = FnPointerRouter::new();

    let callback = |req: &Request| -> Response {
        req.print_info();
        let mut res = Response::success();
        res.set_body(&String::from_utf8_lossy(&req.body));
        res
    };

    router.add_route("/echo", callback);

    let mut req = Request::get_request();
    req.set_url("/echo");
    req.set_body("Who are you?");

    router.handle_request(&req).print_info()
}