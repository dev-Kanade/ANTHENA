use std::process::Command;
use std::process;
pub fn installer(){
    printwelcom();
    chek_postgres();
}

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
    "#;

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
    super::table::table(0);
    create_systemctl();
}


fn postgres_cheak_error(_error: std::io::Error){
    eprintln!("[ERROR]Postgresの確認中にエラーが発生しました。");
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
        .args(["systemctl","enable","{systemname}"])
        .output();
    println!("[INF]サービスを起動中...");
    let _ = Command::new("sudo")
        .args(["systemctl","restart","{systemname}"])
        .output();
    println!("[INF]インストールが完了しました。インストールウェザードを終了します。");
    process::exit(0);
    
}