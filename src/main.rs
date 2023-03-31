use std::{io, cmp::Ordering};
use rand::Rng;

fn main() {
    
    println!("Adivinhe o número!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("O número secreto é {secret_number}");
    loop {
    
        println!("Entre o seu palpite!");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("ocorreu um erro ao ler a linha!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("seu palpite foi {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Muito pequeno!"),
            Ordering::Greater => println!("Muito grande!"),
            Ordering::Equal => {
                println!("Acertou!");
                break;
            }
        }
    }

}
