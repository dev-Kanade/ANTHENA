pub fn recovery(){
    //ここに壊れているファイルを検知するコードを書きます。
}

fn recovery_table(){
    //テーブルの修復
}

fn recovery_config(){
    //構成ファイルの修復
}

fn recovery_user(){
    //システムユーザーの修復
}

fn ceak_postgres() -> bool {
    let output = Command::new("pg_isready")
        .args(["-h", "localhost", "-p", "5432"])  
        .output();

    match output {
        Ok(o) if o.status.success() => {
            true
        }
        Ok(o) => {
            false
        }
        Err(e) => {
            false
        }
    }
}