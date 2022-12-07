use std::collections::HashMap;

#[derive(Clone, PartialEq, Debug)]
enum Json {
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    Array(Vec<Json>),
    Object(Box<HashMap<String, Json>>),
}

impl Json {
    fn from(val: f64) -> Json {
        Json::Number(val)
    }
}

macro_rules! json {
    (null) => { Json::Null };
    ([ $( $element:tt ), * ]) => {
        Json::Array( vec![ $( json!($element) ), *])
    };
    ({ $( $key:tt : $value:tt), *}) => {
        Json::Object(Box::new(vec![
            $( ($key.to_string(), json!($value)) ), *
        ].into_iter().collect()))
    };
    ( $other:tt) => {
        Json::from($other)
    }
}

#[test]
fn test_json_null() {
    assert_eq!(json!(null), Json::Null)
}

#[test]
fn test_json_array() {
    let json_array = json!(
        [{"pitch" : 400.0 }]
    );

    let hand_code_json_array =
    Json::Array(vec![
        Json::Object(Box::new(vec![
            ("pitch".to_string(), Json::Number(400.0))].into_iter().collect()
        ))
    ]);

    assert_eq!(json_array, hand_code_json_array)
}

