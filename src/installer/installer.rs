use std::process::Command;
use std::process;
use std::env;
use std::io;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};

const SYSTEM_USERNAME:&str = "ANTHENA";

/*

\\\ お願い ///
冗長化をして！！！！

*/
pub fn installer(){

    match install_method(){
        1 => {
            install_auto();
        },
        2 => {
            install_custom();
            process::exit(0);
        }
        _ => {
            println!("[ERROR]インストーラーの起動に失敗しました。");
            process::exit(1);
        }
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

fn chek_postgres()-> bool {
    println!("[INF]お使いのデバイスにPostgreSQLがインストールされているかを確認中です。");
    match Command::new("psql").arg("-V").output() {
        Ok(output) if output.status.success() => {
            true
        }
        Ok(_) => {
            false
        }
        Err(e) => {
            if e.kind() == std::io::ErrorKind::NotFound {
                false
            } else {
                println!("[ERROR]ANTHENAインストール中にエラーが発生しました。");
                process::exit(1);
            }
        }
    }
}


//ANTHENAのインストール各処理
/*
fn _setup(){
    println!("[INF]システムのインストールを準備中です...");
    //super::user::create_system_user();
    super::table::create_table();
    create_systemctl();
}
*/

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


fn _create_systemctl(){
    println!("[INF]Systemctlの設定を行っています....");

    let systemname:&str = "anthenaauth.service";
    println!("[INF]System名:{systemname}");
    //この下か上にsystemctlを書く処理を実装する
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
    /* OS以外のチェックも今後実装 */

}

const _DBNAME:&str = "";
fn _create_db(){
    println!("ここでDBに関する操作がおこなわれます。");
    //既存のDBに接続する必要があるため、パスワードがあったらな
}


fn user_exists(username: &str) -> bool {
    let file = match File::open("/etc/passwd") {
        Ok(f) => f,
        Err(_) => return false, 
    };
    
    let reader = BufReader::new(file);

    for line in reader.lines() {
        if let Ok(content) = line {
            if let Some(user) = content.split(':').next() {
                if user == username {
                    return true;
                }
            }
        }
    }
    
    false
    //もちろんDebian系のOS前提
}

fn install_method()->i32{
    println!("ANTHENAをインストールする方法を選択してください。\n1.)推奨される構成で自動インストール\n2.)カスタムインストール");
    println!("インストール番号を入力してください。\n");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("[ERROR]インストーラー起動中にエラーが発生しました。");

    let number:i32 = input.trim().parse().expect("[ERROR]不明なオプションが指定されました。");
    number
}


fn install_auto(){
    printwelcom();
    println!("[INF]デバイス要件を確認中です....");
    match allow_install(){
        true =>{}
        false => {
            println!("[INF]ANTHENAの動作要件をクリアしていないためインストーラーを終了します。");
            process::exit(0);
        }
    }
    match chek_postgres(){
        true => {
            //ここでPostgresのDBにデフォルト設定でアクセスを試みる。
            //アクセスに失敗した場合、ユーザーに尋ねる
        }
        false => {
            install_postgres();
        }
    }
    //PostgreSQLインストール後の流れ
    //Postgresのロールを作成
    //DBを新しく作成
    //テーブルを作成
    //システムユーザーの作成
    match user_exists(SYSTEM_USERNAME){
        true => {
            println!("[WARN]すでにANTHENAユーザーがシステム上に存在します。");
        }
        false => {
            super::user::create_system_user();
        }
    }
    //Systemctlの設定
}

fn install_custom(){}

//2026/04/24 :今日は私の誕生日です。そろそろ、インストーラーを書き上げてメインに移りたいです):