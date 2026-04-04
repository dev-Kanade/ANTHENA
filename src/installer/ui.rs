use std::io::{self, Write};

pub fn print_welcome() {
    println!("ようこそ");
    println!("ANTHENAのインストーラーを準備中です");
}

pub fn print_logo() {
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
    println!("{}", logo);
}

pub fn print_spinner(text: &str) {
    let spinners = vec!["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
    for spinner in spinners.iter().cycle().take(20) {
        print!("\r{} {}", spinner, text);
        io::stdout().flush().unwrap();
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    println!("\r✓ {}", text);
}

pub fn prompt_string(prompt: &str, default: Option<&str>) -> String {
    print!("{}", prompt);
    if let Some(def) = default {
        print!(" [{}] ", def);
    }
    io::stdout().flush().unwrap();

    let stdin = io::stdin();
    let mut line = String::new();
    stdin.read_line(&mut line).unwrap();
    let trimmed = line.trim().to_string();

    if trimmed.is_empty() {
        default.unwrap_or("").to_string()
    } else {
        trimmed
    }
}

pub fn prompt_password(prompt: &str) -> String {
    use std::io::Write;

    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let stdin = io::stdin();
    let mut line = String::new();
    stdin.read_line(&mut line).unwrap();
    line.trim().to_string()
}

pub fn prompt_api_port() -> String {
    println!();
    println!("APIサーバーのポート番号を入力してください");
    let port = prompt_string("ポート番号: ", Some("80"));
    if port.is_empty() {
        "80".to_string()
    } else {
        port
    }
}

pub fn print_option_list(title: &str, options: &[&str]) -> usize {
    println!("{}", title);
    for (i, option) in options.iter().enumerate() {
        println!("  {}. {}", i + 1, option);
    }

    loop {
        print!("選択 (1-{}): ", options.len());
        io::stdout().flush().unwrap();

        let stdin = io::stdin();
        let mut line = String::new();
        stdin.read_line(&mut line).unwrap();

        if let Ok(choice) = line.trim().parse::<usize>() {
            if choice > 0 && choice <= options.len() {
                return choice - 1;
            }
        }
        println!("無効な選択です。もう一度試してください。");
    }
}