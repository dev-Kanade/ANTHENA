use sqlx::postgres::PgPoolOptions;
use sqlx::Row;

pub async fn setup_database(
    user: &str,
    pass: &str,
    port: &str,
    db_name: &str,
) -> anyhow::Result<()> {
    println!();
    println!("テーブルを準備中です...");

    let connection_string = format!(
        "postgres://{}:{}@localhost:{}/{}",
        user, pass, port, db_name
    );

    let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(&connection_string)
        .await?;

    let mut tx = pool.begin().await?;

    match create_tables(&mut *tx).await {
        Ok(_) => {
            tx.commit().await?;
            println!("✓ すべてのテーブルを作成しました");
        }
        Err(e) => {
            tracing::error!("Failed to create tables: {}", e);
            tx.rollback().await?;
            anyhow::bail!("テーブル作成に失敗しました。セットアップをロールバックしています。");
        }
    }

    pool.close().await;
    Ok(())
}

async fn create_tables(tx: &mut sqlx::Transaction<'_, sqlx::Postgres>) -> anyhow::Result<()> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS anthenaauth.client (
            user_id TEXT,
            clientname TEXT,
            clientid TEXT,
            clientsecret TEXT,
            create_date TEXT,
            update_date TEXT,
            scope TEXT,
            candomain TEXT
        );
        "#,
    )
    .execute(&mut **tx)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS anthenaauth.scopelist (
            scopename TEXT
        );
        "#,
    )
    .execute(&mut **tx)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS anthenaauth.accesstoken (
            accesstoken TEXT,
            clientid TEXT,
            create_unix BIGINT,
            invalid_unix BIGINT
        );
        "#,
    )
    .execute(&mut **tx)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS anthenaauth.blockip (
            ip TEXT,
            invalid_unix BIGINT,
            clientid TEXT
        );
        "#,
    )
    .execute(&mut **tx)
    .await?;

    Ok(())
}