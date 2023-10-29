use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Write};

fn main() {
    println!("\n---=== Adivinhe o número! ===---\n");

    loop {
        let mut option = String::new();

        println!("Escolha uma das opções: ");
        println!("  1 - Jogar");
        println!("  2 - Pontoação");
        println!("  Aperte outra para Sair\n");

        io::stdin()
            .read_line(&mut option)
            .expect("Falha ao ler linha");

        match option.trim() {
            "1" => play(),
            "2" => read_score(),
            _ => {
                println!("Obrigado por jogar!");
                break;
            }
        }
    }
}

fn play() {
    let secret_number = rand::thread_rng().gen_range(1..=100);

    let mut score = 100.0;
   
    println!("\nPor favo, entre com o seu palpite.");

    loop {
        if f64::trunc(score * 100.0) / 100.0 <= 0.0 {
            score = 0.0;
            break;
        }

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Falha ao ler linha");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{} é muito baixo! Tente outra vez.", guess),
            Ordering::Greater => println!("{} é muito alto! Tente outra vez.", guess),
            Ordering::Equal => {
                println!("Você acerou!");
                break;
            }
        }

        score = score / 2.0;
    }

    println!("\nSua pontuação foi de {:.2}", score);

    let mut name = String::new();

    println!("Por favor, entre com seu nome: ");

    io::stdin()
        .read_line(&mut name)
        .expect("Falha ao ler linha");

    write_score(name.trim().to_string(), score);

    println!("Pontoção salva!\n");
}

fn write_score(name: String, score: f64) {
    match File::open("score.txt") {
        Ok(file) => {
            let reader = BufReader::new(file);
            let mut new_score_file_content = String::new();
            let mut line_number = 0;
            let mut registered = false;
            for line in reader.lines() {
                line_number += 1;
                let line = line.expect("Falha ao ler linha");
                let partes: Vec<&str> = line.split(':').collect();
                let scote_line: f64 = partes[1].trim().parse().unwrap();

                if 1 != line_number {
                    new_score_file_content.push_str("\n");
                }

                if score >= scote_line && !registered {
                    registered = true;
                    new_score_file_content.push_str(format!("{}: {:.2}\n", name, score).as_str());
                }
            
                new_score_file_content.push_str(line.as_str());
            }

            if !registered {
                new_score_file_content.push_str(format!("\n{}: {:.2}", name, score).as_str());
            }
            
            let mut file = BufWriter::new(File::create("score.txt").unwrap());
            file.write_all(new_score_file_content.as_bytes()).unwrap();
        }
        Err(_) => {
            let mut file = BufWriter::new(File::create("score.txt").unwrap());
            file.write_all(format!("{}: {:.2}\n", name, score).as_bytes())
                .unwrap();
        }
    }
}

fn read_score() {
    println!("\n---=== Pontoção ===---");
    match File::open("score.txt") {
        Ok(file) => {
            let reader = BufReader::new(file);
            let mut line_number = 0;
            for line in reader.lines() {
                line_number += 1;
                println!("{} - {}", line_number, line.unwrap());
            }
        }
        Err(_) => {
            println!("Nenhuma pontuação registrada.");
        }
    }
    println!("---=== ******* ===---\n");
}
