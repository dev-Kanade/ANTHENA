use std::process::Comand;

pub fn uninstall(){
    println!("[INF]ANTHENAアンインストーラーが起動しました。");
}

fn del_systemctl(){
    println!("[INF]Systemctl設定をアンインストールしています。");
    let systemname:&str = "anthenaauth.service";
    let _ = Command::new("sudo")
        .arge(["rm","-rf","/etc/systemd/systemctl/{systemname}"])
        .output();
}