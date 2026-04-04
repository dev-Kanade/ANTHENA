use std::fs;
use std::path::PathBuf;

pub fn create_env_file(
    binary_dir: &PathBuf,
    db_user: &str,
    db_pass: &str,
    db_port: &str,
    db_name: &str,
    api_port: &str,
) -> anyhow::Result<()> {
    let env_path = binary_dir.join(".env");

    let env_content = format!(
        r#"# ANTHENA OAuth Authentication System Configuration

# Database Configuration
DBUSER={}
DBPASS={}
DBPORT={}
DBNAME={}
DBHOST=localhost

# API Configuration
APIPORT={}

# Application
APP_NAME=ANTHENA
APP_ENV=production
LOG_LEVEL=info

# Security
JWT_SECRET=your-secret-key-change-me
SESSION_TIMEOUT=3600
"#,
        db_user, db_pass, db_port, db_name, api_port
    );

    fs::write(&env_path, env_content)?;
    tracing::info!("Created .env file at {:?}", env_path);

    Ok(())
}