# Rust开发

## 安装环境

- 安装 Rust 环境  [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)
- 安装 VSCode [https://code.visualstudio.com/](https://code.visualstudio.com/)
- 安装 VSCode 的 Rust 扩展  [https://marketplace.visualstudio.com/items?itemName=rust-lang.rust](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust)
- 安装 VSCode 的 Rust 调试环境
    - Windows 系统 [https://marketplace.visualstudio.com/items?itemName=ms-vscode.cpptools](https://marketplace.visualstudio.com/items?itemName=ms-vscode.cpptools)
    - Linux/Mac 系统 [https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb)

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

![debug_rs_001](https://user-images.githubusercontent.com/8216630/76219541-f2490980-6250-11ea-8aff-31d0151b10bc.jpg)

- 会初始化配置文件

![debug_rs_002](https://user-images.githubusercontent.com/8216630/76219550-f543fa00-6250-11ea-9cf5-1c9642b132c7.jpg)

- 打断点

![debug_rs_003](https://user-images.githubusercontent.com/8216630/76219560-fa08ae00-6250-11ea-8b22-98301b9969b1.jpg)

- 调试结果

![debug_rs_debug](https://user-images.githubusercontent.com/8216630/76219573-fecd6200-6250-11ea-818a-40e26456c96c.jpg)
