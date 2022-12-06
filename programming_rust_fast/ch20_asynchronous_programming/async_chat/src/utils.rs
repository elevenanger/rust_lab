use std::error::Error;
use serde::de::DeserializeOwned;
use async_std::prelude::*;
use serde::Serialize;
use std::marker::Unpin;

pub type ChatError = Box<dyn Error + Send + Sync + 'static>;
pub type ChatResult<T> = Result<T, ChatError>;

/// 将数据以 json 的形式进行序列化通过 TCPStream 发送
pub async fn send_as_json<S, P>(outbound: &mut S, packet: &P) -> ChatResult<()>
where   S: async_std::io::Write + Unpin,
        P: Serialize,
{
    let mut json = serde_json::to_string(&packet)?;
    json.push('\n');
    outbound.write_all(json.as_bytes()).await?;
    Ok(())
}

/// 对接收到的数据进行反序列化
pub fn receive_as_json<S, P>(inbound: S) -> impl Stream<Item = ChatResult<P>>
where   S: async_std::io::BufRead + Unpin,
        P: DeserializeOwned
{
    inbound.lines()
        .map(|line_result| -> ChatResult<P> {
            let line = line_result?;
            let parsed = serde_json::from_str::<P>(&line)?;
            Ok(parsed)
        })
}