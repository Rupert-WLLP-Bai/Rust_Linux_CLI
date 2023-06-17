# Rust Linux CLI - Matrix 特效

这个 Rust Linux CLI 项目实现了字符在终端中以随机的方式下落并渐变色彩。在终端顶部的中央，还会显示当前日期和时间以及退出提示。

## 实现细节

该项目使用了以下 Rust 库：

- `crossterm`: 用于在终端中进行光标控制、颜色设置等操作。
- `rand`: 用于生成随机数和字符。
- `chrono`: 用于获取当前日期和时间。

关键代码的语法和实现细节如下：

1. 创建 MatrixChar 结构体：

```rust
struct MatrixChar {
    x: u16,
    y: u16,
    color: style::Color,
    char: char,
}
```

MatrixChar 结构体代表下落的字符，包含了字符的位置、颜色和字符本身。

2. 在主循环中，获取当前终端的尺寸：

```rust
let mut terminal_width = terminal::size()?.0;
let mut terminal_height = terminal::size()?.1;
```

使用 `terminal::size()` 函数获取终端的宽度和高度。

3. 更新 MatrixChar 的位置：

```rust
impl MatrixChar {
    fn update(&mut self, terminal_width: u16, terminal_height: u16) {
        self.y += 1;
        if self.y >= terminal_height {
            self.y = 0;
            self.x = rand::thread_rng().gen_range(0..terminal_width);
        }
    }
}
```

`update()` 方法根据终端的尺寸更新字符的位置。如果字符超出终端的底部边界，就将其位置重置到顶部，并随机选择一个新的水平位置。

4. 生成随机颜色：

```rust
fn generate_random_color() -> style::Color {
    let mut rng = rand::thread_rng();
    let color = rng.gen_range(0..=255);
    style::Color::AnsiValue(color)
}
```

`generate_random_color()` 函数使用 `rand` 库生成一个随机的 ANSI 颜色。

5. 生成随机字符：

```rust
let char = rng
    .lock()
    .unwrap()
    .clone()
    .sample_iter(&rand::distributions::Alphanumeric)
    .take(1)
    .map(|c| c as char)
    .collect::<String>()
    .chars()
    .next()
    .unwrap();
```

这段代码生成一个随机字符，通过调用 `sample_iter()` 方法生成一个随机的 `Alphanumeric` 字符迭代器，然后使用 `take(1)` 获取第一个字符，并进行一系列的转换操作，最终得到一个字符。

6. 刷新终端缓冲区：

```rust
shared_stdout.lock().unwrap().flush()?;
```

使用 `flush()` 方法刷新终端缓冲区，确保输出能够及时显示在终端上。

## 使用方法

1. 在终端中，进入项目目录

。
2. 使用 Cargo 命令构建和运行项目：

```
cargo build
cargo run
```

3. 在终端中观察 Matrix 特效的下落效果。

4. 按下 'q' 键退出程序。