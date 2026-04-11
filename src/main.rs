mod installer;
//use std::env;
use calp::{Parser,SubCommand};

fn main() {
    let cheak_system = cheak_anthena_system();
    if cheak_system == true {
    }else if cheak_system == false {
        installer::main::installer(0);//これは、将来的にif文に直してください。
    }else{
        println!("[INF]ANTHENA起動中にエラーが発生しました。");
    }
}

fn cheak_anthena_system()-> bool {
    false
    //今は問答無用でfalseを返してるけどここにインストール済みをチェックするコードを書きます。てか、書いてほしい。
}