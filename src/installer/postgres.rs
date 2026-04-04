use std::process::Command;
use std::path::PathBuf;
use crate::installer::ui;

pub async fn setup_postgres(binary_dir: &PathBuf) -> anyhow::Result<(String, String, String, String)> {
    // PostgreSQL インストール確認
    if is_postgres_installed()? {
        tracing::info!("PostgreSQL is already installed");
        println!("✓ PostgreSQLが検出されました");
        println!();

        // ユーザー・パスワード入力
        let db_user = ui::prompt_string("PostgreSQLユーザー名: ", Some("postgres"));
        let db_pass = ui::prompt_password("PostgreSQLパスワード: ");

        // ポート入力
        let db_port = ui::prompt_string(
            "PostgreSQLポート (デフォルトなら空白のままEnterを押してください): ",
            Some("5432"),
        );

        let databases = list_databases(&db_user, &db_pass, &db_port).await?;
        let db_name = select_database(&databases)?;
        check_and_create_schema(&db_user, &db_pass, &db_port, &db_name).await?;

        Ok((db_user, db_pass, db_port, db_name))
    } else {
        println!("PostgreSQLがインストールされていません。インストール中...");
        println!();

        install_postgres().await?;

        println!();
        println!("✓ PostgreSQLのインストールが完了しました");
        println!();

        let db_user = "postgres".to_string();
        let db_pass = "postgres".to_string();
        let db_port = "5432".to_string();
        let db_name = "anthena".to_string();

        create_database(&db_user, &db_pass, &db_port, &db_name).await?;
        create_schema(&db_user, &db_pass, &db_port, &db_name, "anthenaauth").await?;

        Ok((db_user, db_pass, db_port, db_name))
    }
}

fn is_postgres_installed() -> anyhow::Result<bool> {
    let output = Command::new("which")
        .arg("psql")
        .output();

    Ok(output.is_ok() && output?.status.success())
}

async fn install_postgres() -> anyhow::Result<()> {
    let script = r#"
        set -e
        echo "PostgreSQLをインストール中..."
        
        if [ -f /etc/os-release ]; then
            . /etc/os-release
            if [ "$ID" = "ubuntu" ] || [ "$ID" = "debian" ]; then
                sudo apt-get update
                sudo apt-get install -y postgresql postgresql-contrib
            elif [ "$ID" = "centos" ] || [ "$ID" = "rhel" ] || [ "$ID" = "fedora" ]; then
                sudo dnf install -y postgresql-server postgresql-contrib
                sudo postgresql-setup initdb
                sudo systemctl start postgresql
                sudo systemctl enable postgresql
            elif [ "$ID" = "arch" ]; then
                sudo pacman -S postgresql
                sudo systemctl start postgresql
                sudo systemctl enable postgresql
            fi
        fi
        
        # PostgreSQL デフォルトユーザーパスワード設定（Linux/Unix）
        sudo -u postgres psql -c "ALTER USER postgres PASSWORD 'postgres';" 2>/dev/null || true
    "#;

    let output = Command::new("bash")
        .arg("-c")
        .arg(script)
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        tracing::warn!("PostgreSQL install warning: {}", stderr);
    }

    Ok(())
}

async fn list_databases(
    user: &str,
    pass: &str,
    port: &str,
) -> anyhow::Result<Vec<String>> {
    let psql_cmd = format!(
        "PGPASSWORD='{}' psql -h localhost -U {} -p {} -t -c \"SELECT datname FROM pg_database WHERE datistemplate = false ORDER BY datname;\"",
        pass, user, port
    );

    let output = Command::new("bash")
        .arg("-c")
        .arg(&psql_cmd)
        .output()?;

    let stdout = String::from_utf8(output.stdout)?;
    let databases: Vec<String> = stdout
        .lines()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    Ok(databases)
}

fn select_database(databases: &[String]) -> anyhow::Result<String> {
    println!();
    println!("既存のデータベース一覧:");

    let mut options = vec!["🆕 新規作成 (ANTHENA)".to_string()];
    options.extend(databases.iter().cloned());

    for (i, opt) in options.iter().enumerate() {
        if i == 0 {
            println!("  ◉ {}  (デフォルト)", opt);
        } else {
            println!("  ○ {}", opt);
        }
    }

    println!();
    let choice = ui::print_option_list("使用するデータベースを選択してください", 
        &options.iter().map(|s| s.as_str()).collect::<Vec<_>>());

    if choice == 0 {
        Ok("anthena".to_string())
    } else {
        Ok(options[choice].clone())
    }
}

async fn create_database(
    user: &str,
    pass: &str,
    port: &str,
    db_name: &str,
) -> anyhow::Result<()> {
    let psql_cmd = format!(
        "PGPASSWORD='{}' psql -h localhost -U {} -p {} -c \"CREATE DATABASE {} IF NOT EXISTS;\"",
        pass, user, port, db_name
    );

    let output = Command::new("bash")
        .arg("-c")
        .arg(&psql_cmd)
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Failed to create database: {}", stderr);
    }

    Ok(())
}

async fn check_and_create_schema(
    user: &str,
    pass: &str,
    port: &str,
    db_name: &str,
) -> anyhow::Result<()> {
    let check_cmd = format!(
        "PGPASSWORD='{}' psql -h localhost -U {} -p {} -d {} -t -c \"SELECT schema_name FROM information_schema.schemata WHERE schema_name = 'anthenaauth';\"",
        pass, user, port, db_name
    );

    let output = Command::new("bash")
        .arg("-c")
        .arg(&check_cmd)
        .output()?;

    let stdout = String::from_utf8(output.stdout)?;
    
    if stdout.trim().is_empty() {
        create_schema(user, pass, port, db_name, "anthenaauth").await?;
    } else {
        println!("anthenaauth スキーマが検出されました");
    }

    Ok(())
}

async fn create_schema(
    user: &str,
    pass: &str,
    port: &str,
    db_name: &str,
    schema_name: &str,
) -> anyhow::Result<()> {
    let psql_cmd = format!(
        "PGPASSWORD='{}' psql -h localhost -U {} -p {} -d {} -c \"CREATE SCHEMA IF NOT EXISTS {};\"",
        pass, user, port, db_name, schema_name
    );

    let output = Command::new("bash")
        .arg("-c")
        .arg(&psql_cmd)
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Failed to create schema: {}", stderr);
    }

    println!("✓ {} スキーマを作成しました", schema_name);
    Ok(())
}