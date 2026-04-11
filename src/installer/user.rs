use rand::Rng;
use std::process::Command;
use std::process;

const SYSTEM_USERNAME:&str = "ANTHENA";

pub fn create_system_user(){
    /*
    let _ = Command::new("sudo")
        .args(["adduser",""])
    */
    let user_password:String = create_pass();
    println!("生成したパスワード:{user_password}");

    let create = Command::new("useradd")
        .args([
            "--system",
            "--no-create-home",
            "--shell",
            "/usr/sbin/nologin",
            SYSTEM_USERNAME
        ])
        .status();
    match create {
        Ok(s) if s.success() => {
            println!("[INF]システムユーザーを作成しました。");
        }
        Ok() => {
            eprintln!("[ERROR]システムユーザーの作成に失敗しました。");
            process::exit(1);
        }
        Err() => {
            eprintln!("[ERROR]システムユーザーの作成に失敗しました。");
            process::exit(1);
        }
    }
}


fn create_pass()-> String {
    let pass:String = rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(10)
        .map(char::from)
        .collect();

    pass
}
