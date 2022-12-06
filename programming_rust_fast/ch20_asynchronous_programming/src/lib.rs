use std::{thread};
use argonautica::Hasher;
use async_std::{io, net};
use async_std::net::{TcpListener, TcpStream};
use async_std::prelude::*;
use async_std::task;

/// 异步函数：
/// 函数以 async 关键字开头，
/// 使用 async_std crate 版本的 TcpSteam
///
/// 每次调用返回 future 的函数，代码都会加上 .await
/// 它会等待一个 future 直到它准备好，
/// await 表达式求出 future 的最终值。
///
/// 调用一个异步函数,
/// 异步函数会在执行代码体之前返回，
/// 调用的返回值此时还没有计算出来，
/// 函数返回的只是一个最终值的 future ，
/// 所以执行这个函数返回返回的是一个 io::Result<String> 的 future
///
/// 当第一次 poll cheapo_request_async 返回的 future 时，
/// 将会从函数体开头执行，
/// 执行到第一个 await 表达式 便会 poll connect 的返回值,
/// 如果他还没有准备就绪，
/// 就会返回 Poll::Pending 给调用者，
/// 直到 connect 返回 Poll::Ready 的时候，对 cheapo_request_async 的 poll 才能继续，
///
/// await 表达式获取了 future 的所有权，并对其 poll ,
/// 如果准备就绪了，
/// future 的最终值就是 await 表达式的值，
/// 程序就能继续执行，否则，它会返回 Poll::Pending 给它的调用者
///
/// 关键是，下一次 poll cheapo_request_async future 不是从头开始执行函数，
/// 而是从上次 pending 的地方，即 connect 开始执行，
/// 直到这个 future 准备好之后才会继续开始执行函数剩余的部分
async fn cheapo_request_async(host: &str, port: u16, path: &str)
    -> io::Result<String> {
    let mut socket = TcpStream::connect((host, port)).await?;

    let request = format!("GET {} HTTP/1.1\r\nHost: {}\r\n\r\n", path, host);
    println!("thread => {:?}\n {}",thread::current().id(), request);
    socket.write_all(request.as_bytes()).await?;
    socket.shutdown(net::Shutdown::Write)?;

    let mut response = String::new();
    socket.read_to_string(&mut response).await?;
    println!("response => \n{}", response);

    Ok(response)
}

/// 将参数的引用类型转换成 owned 类型
async fn cheapo_request_async_to_owned(host: String, port: u16, path: String)
    -> io::Result<String> {
    cheapo_request_async(&host, port, &path).await
}

/// 使用 async 代码块在常规的函数中调用执行 async 函数
fn cheapo_request_use_async_block(host: &str, port: u16, path: &str)
    -> impl Future<Output = io::Result<String>> {
    let host = host.to_string();
    let path = path.to_string();

    async move {
        cheapo_request_async(&host, port, &path).await
    }
}

/// task::spawn_local 使用同一个线程发起多个异步请求
async fn multi_request(requests: Vec<(String, u16, String)>)
    -> Vec<io::Result<String>> {

    let mut handles = Vec::new();
    for (host, port, path) in requests {
        handles.push(
            task::spawn_local(cheapo_request_async_to_owned(host, port, path)));
    }

    let mut results = vec![];
    for handle in handles {
        results.push(handle.await);
    }

    results
}

/// 简单的 tcp 服务器
async fn simple_server(addr: String) -> io::Result<()> {
    let listener = TcpListener::bind(addr).await?;
    let mut incoming = listener.incoming();

    while let Some(stream) = incoming.next().await {
        let stream = stream?;
        let handle = task::spawn(stream_handle(stream));
        handle.await?;
    }

    Ok(())

}

/// 将流处理的部分单独出一个异步函数
async fn stream_handle(stream: TcpStream) -> io::Result<()> {
    let (reader, writer) = &mut (&stream, &stream);

    let msg = format!("Request Accepted From : {}\n", stream.peer_addr()?);
    writer.write_all(msg.as_bytes()).await?;
    let msg = format!("{:?} handling ...\n", thread::current().id());
    writer.write_all(msg.as_bytes()).await?;
    io::copy(reader, writer).await?;

    Ok(())
}

#[async_std::test]
async fn test_simple_server() -> io::Result<()> {

    let host = "127.0.0.1";
    let port = 3030;
    let addr = format!("{}:{}", &host, &port);

    task::block_on(simple_server(addr))?;

    Ok(())
}

#[async_std::test]
async fn test_call_cheapo_async() -> io::Result<()> {
    let host = "127.0.0.1";
    let port = 3030;

    for _ in 0..1000 {
        cheapo_request_async(host, port, "/async").await?;
    }

    Ok(())

}

/// 使用 task::block_on 方法从同步函数中调用异步函数
#[test]
fn test_call_cheapo_sync() {
    let host = "ifconfig.me";
    let port = 80;

    let response =
        task::block_on(cheapo_request_async(host, port, "/")).unwrap();

    println!("{}", response);
}

#[test]
fn test_multi_request() {
    let requests = vec![
        ("127.0.0.1".to_string(), 3030, "/".to_string()),
        ("127.0.0.1".to_string(), 3030, "/".to_string()),
        ("127.0.0.1".to_string(), 3030, "/".to_string()),
        ("127.0.0.1".to_string(), 3030, "/".to_string()),
    ];

    let response = task::block_on(multi_request(requests));

    println!("{:?}", response)
}

#[test]
fn test_cheapo_with_async_block() {
    let response =
        task::block_on(
            cheapo_request_use_async_block("ifconfig.me", 80, "/"));
    println!("{}", response.unwrap())
}

/// 使用 task::spawn 函数将一个 future spawn 到线程池中
/// task::spawn 可以使用线程池，多个线程 poll future，
/// task::spawn_local 使用单线程 poll future
#[async_std::test]
async fn test_spawn_tasks_in_thread_pool() -> io::Result<()> {
    let requests = vec![
        ("127.0.0.1", 3030, "/"),
        ("127.0.0.1", 3030, "/"),
        ("127.0.0.1", 3030, "/"),
        ("127.0.0.1", 3030, "/"),
    ];

    let mut handles = vec![];

    for (host, port, url) in requests {
        handles.push(task::spawn(
            cheapo_request_use_async_block(host, port, url)
        ))
    }

    for handle in handles {
            println!("{}", handle.await.unwrap())
    }

    Ok(())
}

/// task::spawn_blocking 使用单独的线程执行耗时计算密集型的任务，
/// 避免和其它的线程频繁争抢 CPU 资源带来的额外的开销
async fn check_pass(pass: &str, hash: &str, key: &str)
    -> Result<bool, argonautica::Error> {
    let password = pass.to_string();
    let hash = hash.to_string();
    let key = key.to_string();

    task::spawn_blocking(move || {
        argonautica::Verifier::default()
            .with_hash(hash)
            .with_password(password)
            .with_secret_key(key)
            .verify()
    }).await
}

#[test]
fn test_check_pass() {
    let password = "123456";
    let key = "something you know.";

    let mut hasher = Hasher::default();

    let pass_hash = hasher
        .with_password(password)
        .with_secret_key(key)
        .hash().unwrap();

    println!("hash => {}", &pass_hash);

    let result =
        task::block_on(check_pass(password, &pass_hash, key));

    println!("result => {}", result.unwrap())
}

async fn many_requests(urls: &[String])
    -> Vec<Result<String, surf::Error>> {
    let client = surf::Client::new();

    let mut handles = vec![];
    for url in urls {
        let request = client.get(&url).recv_string();
        handles.push(async_std::task::spawn(request));
    }

    let mut results = vec![];
    for handle in handles {
        results.push(handle.await);
    }

    results
}

#[test]
fn test_many_requests() {
    let urls = vec![
        "https://ifconfig.me".to_string(),
        "https://www.baidu.com".to_string()
    ];

    let responses =
        task::block_on(many_requests(&urls));

    for response in responses {
        match response {
            Ok(res) => println!("{}", res),
            Err(err) => eprintln!("{}\n", err)
        }
    }
}