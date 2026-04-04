pub fn installer(){
    println!("インストールを実行します");//後で消します。
    printwelcom();

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
}