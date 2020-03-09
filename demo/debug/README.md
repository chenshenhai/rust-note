# Rust开发

## 安装环境

- 安装 Rust 环境
- 安装 VSCode 
- 安装 VSCode 的 Rust 扩展
- 安装 VSCode 的 Rust 调试环境
  - Windows 系统
  - Linux/Mac 系统

## 开始调试

### 初始化调试项目

```sh
mkdir debug_rs

cd debug_rs

cargo init
```

### 编辑调试代码

在 `./debug_rs/src/main.rs` 文件上写入待调试代码

```rust
fn add(x: i32, y: i32) -> i32 {
    return x + y;
}

fn main() {
    let x = 123;
    let y = 456;
    let result = add(x, y);

    let result = add(result, result);
    println!("result = {}", result);
}
```

### 调试配置

- 选择 `调试 -> 添加配置`
- 会初始化配置文件
- 打断点
- 开始调试