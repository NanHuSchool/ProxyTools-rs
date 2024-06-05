# ProxyTools-rs

ProxyTools-rs 是一个用 Rust 编写的工具，用于管理 Windows 机器上的代理设置。这个项目包含两个主要的脚本，一个用于禁用代理设置，另一个用于启用代理设置。

## 功能

- `noproxy.rs`: 禁用代理设置。
- `proxy.rs`: 启用代理设置并配置代理服务器地址和端口。

### 编译和运行
```
cargo build --release
```

### 配置

- `proxy.rs` 中的默认代理服务器地址和端口，代理绕过列表如下。你可以根据需要修改这些值：

    ```rust
    let proxy_address = "your_proxy_address";
    let proxy_port = "your_proxy_port";
    let bypass_list = "your_bypass_list";
    ```

## 注意事项

- 运行这些脚本需要管理员权限，因为它们会修改注册表设置。
- 请在修改脚本之前备份你的注册表设置，以防出现问题。
