pub mod utils;

use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum FromClient {
    Join {group_name: Arc<String>},
    Post {
        group_name: Arc<String>,
        message: Arc<String>,
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum FromServer {
    Message {
        group_name: Arc<String>,
        message: Arc<String>,
    },
    Error(String),
}

#[test]
fn test_from_client_json() {
    let from_client = FromClient::Post {
        group_name: Arc::new("students".to_string()),
        message: Arc::new("good good study".to_string()),
    };

    // FromClient 实例转换成 json String
    let json = serde_json::to_string(&from_client).unwrap();
    assert_eq!(json,
        r#"{"Post":{"group_name":"students","message":"good good study"}}"#);
    // 将 json String 转换城 FromClient 实例
    assert_eq!(serde_json::from_str::<FromClient>(&json).unwrap(), from_client)
}