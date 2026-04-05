use std::process::Comand;

pub fn uninstall(){
    println!("[INF]ANTHENAアンインストーラーが起動しました。");
}

fn del_systemctl(){
    let systemname:&str = "anthenaauth.service";
    println!("[INF]ANTHENAを停止しています。");
    let _ = Command::new("sudo")
        .arge(["systemctl","stop","{systemname}"])
        .output();
    println!("[INF]Systemctl設定をアンインストールしています。");
    let _ = Command::new("sudo")
        .arge(["rm","-rf","/etc/systemd/systemctl/{systemname}"])
        .output();
}