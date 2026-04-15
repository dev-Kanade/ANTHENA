use std::process::Command;

pub fn installer(installertype:i32){
    let _ = Command::new("sudo")
        .args(["-l"])
        .output();
        
    match installertype{
        0 => super::installer::installer();
        1 => super::updater::update::update();
        2 => super::uninstaller::uninstall::uninstall();
        _ => println!("インストーラーの起動に失敗しました。");
    }
}

/* installertypeが0ならインストール,1ならアップデート,2ならアンインストールパッケージを実行します。*/