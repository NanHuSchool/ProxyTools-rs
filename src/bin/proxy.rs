use std::process::Command;

fn main() {
    // 代理地址
    let proxy_address = "";
    // 代理服务器端口
    let proxy_port = "";
    // 绕过放行列表
    let bypass_list = "localhost";

    let proxy_server = format!("{}:{}", proxy_address, proxy_port);

    // 设置代理服务器地址和端口
    Command::new("reg")
        .args(&[
            "add",
            r"HKCU\Software\Microsoft\Windows\CurrentVersion\Internet Settings",
            "/v",
            "ProxyServer",
            "/d",
            &proxy_server,
            "/f",
        ])
        .output()
        .expect("Failed to execute command");

    // 启用代理
    Command::new("reg")
        .args(&[
            "add",
            r"HKCU\Software\Microsoft\Windows\CurrentVersion\Internet Settings",
            "/v",
            "ProxyEnable",
            "/t",
            "REG_DWORD",
            "/d",
            "1",
            "/f",
        ])
        .output()
        .expect("Failed to execute command");

    // 设置代理绕过列表
    Command::new("reg")
        .args(&[
            "add",
            r"HKCU\Software\Microsoft\Windows\CurrentVersion\Internet Settings",
            "/v",
            "ProxyOverride",
            "/d",
            bypass_list,
            "/f",
        ])
        .output()
        .expect("Failed to execute command");

    // 刷新DNS缓存
    Command::new("ipconfig")
        .arg("/flushdns")
        .output()
        .expect("Failed to execute command");

    println!("Proxy settings updated.");
}