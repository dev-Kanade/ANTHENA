use std::process::Command;

pub fn recovery(){
    //ここに壊れているファイルを検知するコードを書きます。
    let state_postgres:bool = chek_postgres();
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
/*
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
    //現在はDefaultポートのみだが、今後は.envから読み取れるようにする。
}
*/
fn recovery_postgres(){
    //ここではPostgreSQLの回復環境を呼び出す予定です。
}