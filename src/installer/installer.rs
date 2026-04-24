use std::process::Command;
use std::process;
use std::env;
//use std::fs;
use std::io;

const _SYSTEM_USERNAME:&str = "ANTHENA";

pub fn installer(){

    match install_method(){
        1 => {
            install_auto();
        },
        2 => {
            println!("[INF]カスタムインストーラー");
            println!("制作中");
            process::exit(0);
        }
        _ => {
            println!("エラー");
            process::exit(1);
        }
    }
   /* let allow:bool = allow_install();
    if allow == true {
        printwelcom();
        chek_postgres();
    }else if allow ==false {
        println!("お使いのデバイスはANTHENAをインストールする要件が不足しています。");
        process::exit(0);
    }else{
        println!("[ERROR]ANTHENAインストール中にエラーが発生しました。");
        process::exit(1);
    }
        ||||if分からmatch分に切り替えるためコメントアウトしています。||||
        */
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


fn create_systemctl(){
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


fn _user_exists(username:&str)->io::Result<bool>{
    /*
    let file = fs::File::open("/etc/passwd")?;

    let reader = io::BufReader::new(file);

    for line in reader.lines(){
        let line = line?;
        if let Some(name) = line.split(":").next(){
            if name == username { 
                return Ok(true);
            }
        }
    }
    Ok(false)
    */
    Ok(true)
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
        true => {}
        false => {
            install_postgres();
        }
    }
}

fn _install_custom(){}