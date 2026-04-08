mod installer;
use std::env;

use actix_web::{web,App,HttpServer,HttpRequest,HttpResponse};
use reqwest::Client;

fn main() {
    let cheak_system = cheak_anthena_system();
    if cheak_system == true {
        api_server();
    }else if cheak_system == false {
        installer::main::installer(1);//これは、将来的にif文に直してください。
    }else{
        println!("[INF]ANTHENA起動中にエラーが発生しました。");
    }
    api_server();
}

fn cheak_anthena_system()-> bool {
    true
    //今は問答無用でfalseを返してるけどここにインストール済みをチェックするコードを書きます。てか、書いてほしい。
}
#[actix_web::main]
async fn api_server()->std::io::Result<()>{
    HttpServer::new(||{
        App::new()
    })
    .bind("0.0.0.0::8080")?
    .run()
    .await;
    println!("[INF]API起動中....");//仮
    loop{}
}

fn cheak_os()->bool {
    //メモ:対応OSかどうかのみboolで返します。
    true
}