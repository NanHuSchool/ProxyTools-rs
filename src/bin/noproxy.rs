use std::process::Command;

fn main() {
    // 删除代理服务器地址和端口设置
    Command::new("reg")
        .args(&[
            "delete",
            r"HKCU\Software\Microsoft\Windows\CurrentVersion\Internet Settings",
            "/v",
            "ProxyServer",
            "/f",
        ])
        .output()
        .expect("Failed to execute command");

    // 禁用代理
    Command::new("reg")
        .args(&[
            "add",
            r"HKCU\Software\Microsoft\Windows\CurrentVersion\Internet Settings",
            "/v",
            "ProxyEnable",
            "/t",
            "REG_DWORD",
            "/d",
            "0",
            "/f",
        ])
        .output()
        .expect("Failed to execute command");

    // 删除代理绕过列表
    Command::new("reg")
        .args(&[
            "delete",
            r"HKCU\Software\Microsoft\Windows\CurrentVersion\Internet Settings",
            "/v",
            "ProxyOverride",
            "/f",
        ])
        .output()
        .expect("Failed to execute command");

    // 刷新DNS缓存
    Command::new("ipconfig")
        .arg("/flushdns")
        .output()
        .expect("Failed to execute command");

    println!("Proxy settings have been cleared and disabled.");
}