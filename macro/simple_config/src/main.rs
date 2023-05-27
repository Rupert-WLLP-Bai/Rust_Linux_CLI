use std::collections::HashMap;


trait Config {
    fn parse_config(config: &HashMap<String, String>) -> Result<Self, String>
    where
        Self: Sized;
}

#[derive(Debug)]
#[allow(dead_code)]
struct MyConfig {
    name: String,
    age: u32,
    is_enabled: bool,
}

impl Config for MyConfig {
    fn parse_config(config: &HashMap<String, String>) -> Result<Self, String> {
        let name = config
            .get("name")
            .ok_or_else(|| "Missing field: name".to_string())?
            .parse()
            .map_err(|_| "Failed to parse field: name".to_string())?;

        let age = config
            .get("age")
            .ok_or_else(|| "Missing field: age".to_string())?
            .parse()
            .map_err(|_| "Failed to parse field: age".to_string())?;

        let is_enabled = config
            .get("is_enabled")
            .ok_or_else(|| "Missing field: is_enabled".to_string())?
            .parse()
            .map_err(|_| "Failed to parse field: is_enabled".to_string())?;

        Ok(Self {
            name,
            age,
            is_enabled,
        })
    }
}

fn main() {
    // 测试代码
    let mut config = HashMap::new();
    config.insert("name".to_string(), "Alice".to_string());
    config.insert("age".to_string(), "20".to_string());
    config.insert("is_enabled".to_string(), "true".to_string());
    config.insert("is_enabled".to_string(), "false".to_string());

    // 通过 Config trait 来解析配置
    let my_config = MyConfig::parse_config(&config).unwrap();
    println!("{:?}", my_config);

}
