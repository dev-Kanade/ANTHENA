use std::process::Command;

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
    //ここにデバイスにPostgresがインストールされているか確認するソースを書きます。
    println!("[INF]お使いのデバイスにPostgreSQLがインストールされているかを確認中です。");
    match Command::new("psql").arg("-V").output() {
        Ok(output) if output.status.success() => {
            setup();
        }
        Ok(_) => {
            install_postgres(); 
        }
        Err(e) => {
            if e.kind() == std::io::ErrorKind::NotFound {
                install_postgres();
            } else {
                postgres_cheak_error(e);
            }
        }
    }
}

fn setup(){
    println!("インストール中です。");
}

fn install_postgres(){
    println!("Postgresをインストール中");
}

fn postgres_cheak_error(_error: std::io::Error){
    eprintln!("インストール中二エラーが発生しました。");
}