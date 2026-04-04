use std::process::Command;
use rand::Rng;
use std::env;
use super::user::create_system_user;
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
}


fn postgres_cheak_error(_error: std::io::Error){
    eprintln!("[ERROR]インストール中にエラーが発生しました。");
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
