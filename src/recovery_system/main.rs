pub fn recovery(){
    //ここに壊れているファイルを検知するコードを書きます。
}

fn recovery_table(){
    //テーブルの修復
}

fn recovery_config(){
    //構成ファイルの修復
}

fn recovery_user(){
    //システムユーザーの修復
}

fn chek_postgres()-> bool{
    match Command::new("psql").arg("-V").output() {
        Ok(output) if output.status.success() => {
            setup();
        }
        Ok(_) => {
            install_postgres();
            setup();
        }
        Err(e) => {
            if e.kind() == std::io::ErrorKind::NotFound {
                install_postgres();
                setup();
            } else {
                postgres_cheak_error(e);
            }
        }
    }
}