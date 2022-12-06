#![warn(rust_2018_idioms)]
#![allow(elided_lifetimes_in_paths)]

use async_chat::utils::ChatResult;
use async_std::prelude::*;
use std::sync::Arc;

mod connection;
mod group_table;
mod group;

use connection::serve;

fn main() -> ChatResult<()> {
    let address = std::env::args().nth(1).expect("用法: server address");

    let chat_group_table = Arc::new(group_table::GroupTable::new());

    async_std::task::block_on(
        async {
            use async_std::{net, task};

            let listener = net::TcpListener::bind(address).await?;

            let mut new_connections = listener.incoming();
            while let Some(socket_result) = new_connections.next().await {
                let socket = socket_result?;
                let groups = chat_group_table.clone();
                task::spawn(async {
                    log_error(serve(socket, groups).await);
                });
            }
            Ok(())
        })
}

fn log_error(result: ChatResult<()>) {
    if let Err(error) = result {
        eprintln!("Error: {}", error);
    }
}