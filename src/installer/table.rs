use tokio_postgres::{Client,NoTls,Error};

#[tokio::main]
pub async fn table(option:i32){
    if option == 0 {
        create_table();
    }else if option == 1 {
        del_table();
    }else{
        println!("[ERROR]不明なテーブル操作です。");
    }
}

fn create_table(){
    println!("[INF]テーブルをセットアップ中です...");
    
}

fn del_table(){
    println!("[INF]テーブルを削除中です...");
}
/* 以下の変数の値や変数名は変更しないでください。変更した場合、システムが動かなくなる可能性があります。 */

const _CLIENT:&str = "anthena_client_list";
const _ACCESSTOKEN:&str = "anthena_accesstoken";
const _LONG_TOKEN:&str = "anthena_long_token";
const _ANTHENA_CONFIG:&str = "anthena_config";

/* ==================== */

/* ここから下は各テーブルを作成する関数を定義します。 */
