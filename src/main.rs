mod installer;
fn main() {
    let cheak_system = cheak_anthena_system();
    if cheak_system == true {
        println!("[MEM]インストール済み");
    }else if cheak_system == false {
        installer::main::installer(0);
    }else{
        println!("ANTHENA起動中にエラーが発生しました。");
    }
    installer::main::installer(0);
}

fn cheak_anthena_system()-> bool {

}