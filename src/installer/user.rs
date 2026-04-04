pub fn create_system_user() {
    let username = "anthena".to_string();

    let userpassword = generate_random_password(10);
    dotenv::dotenv().ok(); 

    let db_user = env::var("DBUSER").unwrap_or_else(|_| {
        "default_dbuser".to_string()
    });

    let db_password = env::var("DBPASSWORD").unwrap_or_else(|_| {
        eprintln!("警告: .env に DBPASSWORD がありません。空文字列を使用します。");
        String::new()
    });

    env::set_var("DBUSER", &db_user);
    env::set_var("DBPASSWORD", &db_password);


    if let Err(e) = Command::new("useradd")
        .args([
            "--system",
            "--no-create-home",
            "--shell", "/usr/sbin/nologin",
            &username,
        ])
        .status()
    {
        eprintln!("エラー: useradd コマンド実行失敗: {}", e);
        return;
    }

    let chpasswd_input = format!("{}:{}", username, userpassword);
    if let Err(e) = Command::new("chpasswd")
        .arg("--crypt-method")
        .arg("SHA512")
        .input(chpasswd_input.as_bytes())
        .status()
    {
        eprintln!("エラー: chpasswd 実行失敗: {}", e);
        return;
    }

    if let Err(e) = Command::new("usermod")
        .args(["-aG", "sudo", &username])
        .status()
    {
        eprintln!("エラー: usermod (sudo追加) 実行失敗: {}", e);
        return;
    }
}

fn generate_random_password(length: usize) -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";

    let mut rng = rand::thread_rng();
    (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}