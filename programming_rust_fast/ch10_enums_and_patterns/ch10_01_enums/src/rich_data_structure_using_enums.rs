mod rich_data_structure_using_enums {
    use std::collections::HashMap;

    /// 使用 Rust 枚举实现丰富数据结构，
    /// JSON 数据结构。
    #[derive(Debug)]
    enum Json {
        Null,
        Boolean(bool),
        Number(f64),
        String(String),
        Array(Vec<Json>),
        /* 使用 Box 是为了 JSON 的值更加紧凑，
            因为枚举类型值中在内存中的长度是相同的
            Box<HashMap> 只占一个机器字，
            指向堆上分配的内存 */
        Object(Box<HashMap<String, Json>>),
    }

    #[test]
    fn test_json_object() {
        let mut profile_info = HashMap::new();
        profile_info.insert("name".to_string(), Json::String("An".to_string()));
        profile_info.insert("married".to_string(), Json::Boolean(true));
        profile_info.insert("height".to_string(), Json::Number(177.5));
        profile_info.insert("education".to_string(), Json::Array(vec![
            Json::String("SuiNing middle high school".to_string()),
            Json::String("College of engineering".to_string())]));
        profile_info.insert("disease".to_string(), Json::Null);

        let an_profile = Json::Object(Box::new(profile_info));

        println!("An's profile => {:?}", an_profile);
    }
}