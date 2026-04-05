mod installer;
fn main() {
    let cheak_system = cheak_anthena_system();
    if cheak_system == true {
        println!("[MEM]インストール済み");
        api_server();
    }else if cheak_system == false {
        installer::main::installer(0);
    }else{
        println!("ANTHENA起動中にエラーが発生しました。");
    }
}

fn cheak_anthena_system()-> bool {
    false
    //今は問答無用でfalseを返してるけどここにインストール済みをチェックするコードを書きます。てか、書いてほしい。
}

fn api_server(){
    println!("[INF]API起動中....");//仮
    loop{}
}