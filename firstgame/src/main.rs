use std::{
    io::{self, stdin, stdout, Read},
};

use crossterm::{execute, terminal::Clear};
use rand::{thread_rng, Rng};

fn main() {



    let mut points: i32 = 0;
    let mut attemps: i32 = 0;
    loop {
        execute!(stdout(), Clear(crossterm::terminal::ClearType::All)).unwrap();
        println!("Tenha sorte\n\n");
        println!("Score: {}", points);
        println!("Pressione J para jogar ou R para finalizar");
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Erro ao ler linha");
        input.make_ascii_lowercase();

        let normalized_input : &str = input.trim();
        attemps += 1;
        match normalized_input {
            "r" => {
                let score = points * (21 - attemps);
                println!("Sua pontuação final foi: {}", score);
                return;
            }
            "j" => {
                let num = thread_rng().gen_range(1..=21);
                let total = points + num;
                if total > 21 {
                    println!("O total deu {}, você perdeu", total);
                    return;
                }

                points = total;
            }
            _ => {
                
            }
        }
        
    }


}
