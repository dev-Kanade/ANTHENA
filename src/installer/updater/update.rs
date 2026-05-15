pub fn update(){
    /* アップデーターはまだ実装しないのでパニックしときます。*/
    panic!("アップデーターの起動に失敗しました。");
    /* =========================================*/
    println!("更新データを確認中...");
    let update = cheak_update();
    if update == true {
        println!("更新データが見つかりました。\nアップデートしますか？(y/n)");
    }else{
        println!("更新データが見つかりませんでした。\n最新バージョンです。");
    }
}

fn cheak_update()->bool{
    true
}
