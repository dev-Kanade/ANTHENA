pub fn update(){
    println!("更新データを確認中...");
    let mut update = cheak_update();
    if update == true {
        println!("更新データが見つかりました。\nアップデートしますか？(y/n)");
    }else{
        println!("更新データが見つかりませんでした。\n最新バージョンです。");
    }
}

fn cheak_update()->bool{
    true
}