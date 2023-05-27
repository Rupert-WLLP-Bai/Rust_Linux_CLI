# Config Parsing Library

这个 Rust 库提供了一个用于解析配置信息的通用框架。它使用了自定义的派生宏和 trait 来简化配置解析的过程。

## 使用示例

以下是一个使用该库的示例：

```rust
mod config_derive;

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

#[derive(Config)]
struct MyConfig;

impl Config for MyConfig {
    fn parse_config(config: &HashMap<String, String>) -> Result<Self, String> {
        // 解析配置字段并返回结构体实例
        // ...
    }
}

fn main() {
    let mut config = HashMap::new();
    // 填充配置信息到 HashMap

    // 解析配置
    let my_config = MyConfig::parse_config(&config).unwrap();
    println!("{:?}", my_config);
}
```

上述代码演示了如何使用 Config trait 和自定义的 MyConfig 结构体来解析配置信息。你需要实现 Config trait 的 parse_config 方法来定义解析逻辑，并通过 derive_config 宏为结构体生成解析方法的实现。在 main 函数中，你可以创建一个 HashMap 来填充配置信息，并调用 parse_config 方法来解析配置并获得结构体的实例。

## 自定义配置结构体

你可以根据你的需求定义自己的配置结构体，并按照以下步骤进行操作：

1. 创建一个新的结构体来表示你的配置信息，为它添加相应的字段。
2. 实现 Config trait 来为结构体定义解析方法。
3. 使用 #[derive(Config)] 宏为结构体生成解析方法的实现。

确保在配置信息中的键和结构体字段名称匹配，并正确处理字段类型和错误情况。详细的示例代码可以在示例文件中找到。

## 配置信息格式

该库假设配置信息以 HashMap<String, String> 的形式传递，其中键是配置字段的名称，值是字符串表示的配置值。
