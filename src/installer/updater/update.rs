pub fn update(){
    println!("更新データを確認中...");
    let mut update = cheak_update();
    println!("{update}");
}

fn cheak_update()->&str{
    let update_size:&str = 12Mb;
    let update_version:&str = 1.0;
    "更新データが見つかりました。\nアップデートサイズ:{update_size}\nバージョン:{update_version}"
}