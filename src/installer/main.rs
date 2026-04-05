use std::prosecc::Command;

pub fn installer(installertype:i32){
    let _ = Command::new("sudo")
        .args(["-l"])
        .output();
    if installertype == 0 {
        super::installer::installer();
    }else if installertype == 1 {
        super::updater::update::update();
    }else if installertype == 2 {
        super::uninstaller::uninstall::uninstall();
    }else{
        eprintln!("インストーラーの起動に失敗しました。");
    }

}

/* installertypeが0ならインストール,1ならアップデート,2ならアンインストールパッケージを実行します。*/