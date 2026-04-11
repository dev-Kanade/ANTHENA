use rand::Rng;
pub fn create_system_user(){
    println!("[MEM]この関数でUbuntuのユーザーを作成します。");
    /*
    let _ = Command::new("sudo")
        .args(["adduser",""])
    */
    let user_password:String = create_pass();
    println!("生成したパスワード:{user_password}");
}


fn create_pass()-> String {
    let pass:String = rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(10)
        .map(char::from)
        .collect();

    pass
}