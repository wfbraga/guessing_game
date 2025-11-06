use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    loop {
        println!("Adivinhe o número!");   

        println!("De o seu chute: digite um número entre 1 e 100. Ou Ctrl+C para sair.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Falha ao ler a linha");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Você chutou: {}", guess);

        match guess.cmp(&secret_number){
            Ordering::Less => println!("Muito baixo!"),
            Ordering::Greater => println!("Muito alto!"),
            Ordering::Equal => {
                println!("Parabéns! Você acertou!");
                break;
            }
        }
    }
}
