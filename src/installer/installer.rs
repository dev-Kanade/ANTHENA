use std::process::Command;
use std::process;
use std::env;
pub fn installer(){
    let allow:bool = allow_install();
    if allow == true {
        printwelcom();
        chek_postgres();
    }else if allow ==false {
        println!("お使いのデバイスはANTHENAをインストールする要件が不足しています。");
        process::exit(0);
    }else{
        println!("[ERROR]ANTHENAインストール中にエラーが発生しました。");
    }
}
/*
インストーラーを書く人へ
今はUbuntuしかサポートしていません。
今後別のディストリビューションへのサポートや別のOSへのサポートをする場合、OS検知と、サポート外OSへのインストール拒否を実装してください。
*/
fn printwelcom(){
    let logo = r#"
    ╔═══════════════════════════════════╗
    ║                                   ║
    ║          A N T H E N A            ║
    ║                                   ║    
    ║        認証・認可システム         ║
    ║                                   ║
    ║     Authentication & Auth.        ║
    ║                                   ║
    ╚═══════════════════════════════════╝
    "#;//絶対に手を加えないで

    println!("{logo}");
    println!("Welcom to ANTHENA Instaler!!");
    println!("[INF]ANTHENAインストーラーを起動しました。")
}

fn chek_postgres(){
    println!("[INF]お使いのデバイスにPostgreSQLがインストールされているかを確認中です。");
    match Command::new("psql").arg("-V").output() {
        Ok(output) if output.status.success() => {
            setup();
        }
        Ok(_) => {
            install_postgres();
            setup();
        }
        Err(e) => {
            if e.kind() == std::io::ErrorKind::NotFound {
                install_postgres();
                setup();
            } else {
                postgres_cheak_error(e);
            }
        }
    }
}

fn setup(){
    println!("[INF]システムのインストールを準備中です...");
    super::user::create_system_user();
   // super::table::table(0);
    create_systemctl();
}


fn postgres_cheak_error(_error: std::io::Error){
    eprintln!("[ERROR]Postgresの確認中にエラーが発生しました。");
    eprintln!("[INF]インストーラーを停止します。");
    process::exit(1);
}


fn install_postgres() {
    println!("[INF]Postgresをインストール中...");
    let _ = Command::new("sudo")
        .args(["apt", "update"])
        .output();

    let _ = Command::new("sudo")
        .args(["apt", "install", "-y", "postgresql", "postgresql-contrib"])
        .output();

    let _ = Command::new("sudo")
        .args(["systemctl", "enable", "postgresql"])
        .output();

    let _ = Command::new("sudo")
        .args(["systemctl", "start", "postgresql"])
        .output();

    println!("[INF]PostgreSQLのインストールに成功しました。");
}


fn create_systemctl(){
    println!("[INF]Systemctlの設定を行っています....");

    let systemname:&str = "anthenaauth.service";
    println!("[INF]System名:{systemname}");

    println!("[INF]サービスを読み込み中");
    let _ = Command::new("sudo")
        .args(["systemctl", "daemon-reload"])
        .output();
    println!("[INF]自動起動を設定中...");
    let _ = Command::new("sudo")
        .args(["systemctl","enable",systemname])
        .output();
    println!("[INF]サービスを起動中...");
    let _ = Command::new("sudo")
        .args(["systemctl","restart",systemname])
        .output();
    println!("[INF]インストールが完了しました。インストールウェザードを終了します。");
    process::exit(0);
    
}

fn allow_install()->bool{
    let os:&str = env::consts::OS;
    if os == "linux" {
        let apt_available: bool = Command::new("which")
            .arg("apt")
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false);
        apt_available
    }else{
        false
    }
}

/* Debian系のAPTパッケージが扱えるOSのみ対応 */

//メモ:apt -v ではなく、apt update APTパッケージが使われているかを確認します。