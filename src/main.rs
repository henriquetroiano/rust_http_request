use std::io;
use futures::executor::block_on;

fn get_user_data(data: String) {
    // println!("{}", data);
    let mut body = reqwest::get("https://www.rust-lang.org");
    println!("body = {:?}", body);

}

fn main() {
    println!("********************");
    println!("**************************");
    println!("********************************");
    println!("**************************************");

    println!("Por favor, digite seu nome de usuário.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

        let internet_data = get_user_data(guess);


        println!("Ok... Verificando...");


        block_on(internet_data);
    
        println!("**************************************");
        println!("********************************");
        println!("**************************");
        println!("********************");
    // println!("You guessed: {}", guess);
}