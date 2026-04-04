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
}