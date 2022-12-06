use std::sync::Arc;
use async_std::prelude::*;
use async_chat::utils::{self, ChatResult};
use async_std::io;
use async_std::net;
/// 从命令行读取客户端的请求发送到服务端
async fn send_commands(mut to_server: net::TcpStream)
    -> ChatResult<()> {
    println!("Commands:\n\
                join GROUP\n\
                post GROUP MESSAGE...\n\
                Type Control-D (on Unix) or Control-Z(on Windows) \
                to close the connection.");

    let mut command_lines = io::BufReader::new(io::stdin()).lines();
    while let Some(command_result) = command_lines.next().await {
        let command = command_result?;

        let request = match parse_command(&command) {
            Some(request) => request,
            None => continue
        };
        utils::send_as_json(&mut to_server, &request).await?;
        to_server.flush().await?
    }
    Ok(())
}

/// 将标准输入的命令内容解析为请求
fn parse_command(line: &str) -> Option<FromClient> {
    let (command, rest) = get_next_token(line)?;

    return if command == "post" {
        let (group, rest) = get_next_token(rest)?;
        let message = rest.trim_start().to_string();
        Some(FromClient::Post {
            group_name: Arc::new(group.to_string()),
            message: Arc::new(message),
        })
    } else if command == "join" {
        let (group, rest) = get_next_token(rest)?;
        if !rest.trim_start().is_empty() {
            return None;
        }
        Some(FromClient::Join {
            group_name: Arc::new(group.to_string()),
        })
    } else {
        eprintln!("Unrecongnized command {:?}", line);
        None
    }
}

/// 将输入 input 拆分成两部分，
/// token 是 input 中第一个不为空白字符的字符，
/// rest 是剩余部分的内容，
/// 如果字符串没有非空白字符则返回 None
fn get_next_token(mut input: &str) -> Option<(&str, &str)> {
    input = input.trim_start();

    if input.is_empty() {
        return None
    }

    match input.find(char::is_whitespace) {
        Some(space) => Some((&input[0..space], &input[space..])),
        None => Some((input, "")),
    }
}

/// 处理从 server 返回的数据
async fn handle_replies(from_server: net::TcpStream) -> ChatResult<()> {
    let buffered = io::BufReader::new(from_server);
    let mut reply_stream =
        utils::receive_as_json(buffered);

    while let Some(reply) = reply_stream.next().await {
        match reply? {
            FromServer::Message { group_name, message} => {
                println!("Message posted to {} : {}", group_name, message);
            }
            FromServer::Error(message) => {
                println!("error from server: {}", message);
            }
        }
    }

    Ok(())

}

use async_std::task;
use async_chat::{FromClient, FromServer};

fn main() -> ChatResult<()> {
    let address = std::env::args().nth(1)
        .expect("用法： client Address:port");

    task::block_on(async {
        let socket = net::TcpStream::connect(address).await?;
        socket.set_nodelay(true)?;

        let to_server = send_commands(socket.clone());
        let from_server = handle_replies(socket);

        from_server.race(to_server).await?;

        Ok(())
    })
}