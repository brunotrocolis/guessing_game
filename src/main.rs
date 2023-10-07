use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Adivinhe o número!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    let mut score = 100.0;

    loop {

        if f64::trunc(score  * 100.0) / 100.0 <= 0.0 {
            score = 0.0;
            break;
        }

        println!("Por favo, entre com o seu palpite.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Falha ao ler linha");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Seu palpite: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Muito baixo!"),
            Ordering::Greater => println!("Muito alto!"),
            Ordering::Equal => {
                println!("Você acerou!");
                break;
            }
        }

        score = score/2.0;
    }
    println!("Sua pontuação foi de {:.2}", score);
}
