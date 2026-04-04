mod installer;
mod uninstaller;
mod updater;

pub fn installer(installertype:i32){
    println!("Hello World!");

    if installertype == 0 {
        installer::installer();
    }else if installertype == 1 {
        updater::update();
    }else if installertype == 2 {
        uninstaller::unintall::unintall();
    }else{
        eprintln!("インストーラーの起動に失敗しました。");
    }

}

/* installertypeが0ならインストール,1ならアップデート,2ならアンインストールパッケージを実行します。*/