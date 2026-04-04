use std::io::{self, BufRead};
use std::path::PathBuf;
use std::env;

mod postgres;
mod database;
mod env_config;
mod ui;

pub async fn run() -> anyhow::Result<()> {
    // ウェルカムメッセージ
    ui::print_welcome();
    
    // ANTHENAアスキーアート
    ui::print_logo();
    
    println!();
    println!("セットアップに必要な依存関係をチェックしています");
    println!();

    let binary_dir = env::current_exe()?
        .parent()
        .ok_or_else(|| anyhow::anyhow!("Could not determine binary directory"))?
        .to_path_buf();

    // PostgreSQLの有無
    let (db_user, db_pass, db_port, db_name) = postgres::setup_postgres(&binary_dir).await?;

    // テーブル作成
    database::setup_database(&db_user, &db_pass, &db_port, &db_name).await?;

    // APIポート設定
    let api_port = ui::prompt_api_port();

    // .env生成
    env_config::create_env_file(
        &binary_dir,
        &db_user,
        &db_pass,
        &db_port,
        &db_name,
        api_port,
    )?;

    println!();
    println!("インストールが完了しました！");
    println!("アプリケーションを再起動してサーバーを開始してください。");

    Ok(())
}