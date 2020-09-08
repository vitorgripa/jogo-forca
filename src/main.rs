use std::fs;
use std::vec::Vec;
use rand::Rng;
use std::io;

fn listar_animais() -> Vec<Vec<char>> {
    let arquivo = fs::read_to_string("./data/animais.txt").expect("Não foi possível abrir o arquivo!");

    let mut animais: Vec<Vec<char>> = Vec::new();
    
    for linha in arquivo.split("\n"){
        animais.push(linha.to_string().to_lowercase().chars().collect());
    }

    animais
}

fn preencher_acertos(acertos: &mut Vec<char>, palavra: &Vec<char>) {
    for _ in palavra {
        acertos.push('_');
    }
}

fn chechar_acertos(acertos: &mut Vec<char>, palpite: &char, palavra: &Vec<char>) -> bool {
    let mut acertou = false;
    
    for (idx, letra) in palavra.iter().enumerate() {
        if letra == palpite {
            acertos[idx] = letra.clone();          
            acertou = true;
        }
    }

    acertou
}

fn checar_palavra(palpite: &Vec<char>, palavra: &Vec<char>) -> bool {
    palpite == palavra
}

fn escolher_palavra(animais: Vec<Vec<char>>) -> Vec<char> {
    let mut rng = rand::thread_rng();
    animais[rng.gen_range(0, animais.len())].clone()
}

fn mostrar_acertos_tela(acertos: &Vec<char>) {
    println!("");
    for caracter in acertos {
        print!("{}", caracter);
        print!(" ");
    }
    println!("\n");
}

fn mostrar_palpites_anteriores_tela(palpites_anteriores: &Vec<char>) {
    print!("[");
    for palpite_anterior in palpites_anteriores {
        print!(" ");
        print!("{}", palpite_anterior);
        print!(" ");
    }
    println!("]");
}

fn verificar_palpites_anteriores(palpites_anteriores: &Vec<char>, palpite: char) -> bool {
    let mut achou = true;

    for palpite_anterior in palpites_anteriores {
        if palpite_anterior == &palpite {
            achou = false;
        }
    }

    achou
}

fn limpar_terminal() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    print!("{}[2J", 27 as char);
}

fn main() {
    limpar_terminal();

    let mut acertos: Vec<char> = vec![];

    let mut chances: u8 = 5;

    let animais = listar_animais();

    let palavra = escolher_palavra(animais);

    let mut palpites_anteriores: Vec<char> = vec![];

    preencher_acertos(&mut acertos, &palavra);
    
    mostrar_acertos_tela(&acertos);

    println!("A dica é:\nÉ um animal!");

    loop {

        if chances == 0 {
            println!("Você perdeu!");
            break
        } else {
            let mut palpite = String::new();

            io::stdin()
                .read_line(&mut palpite)
                .expect("Não foi possível ler seu palpite");
                
            limpar_terminal();
            
            let palpite: Vec<char> = palpite.trim().chars().collect();

            if palpite.len() < 2 {
                let palpite = palpite[0];

                if verificar_palpites_anteriores(&palpites_anteriores, palpite) {
                    let acertou = chechar_acertos(&mut acertos, &palpite, &palavra);
                    palpites_anteriores.push(palpite);
    
                    if !acertou {
                        println!("Você errou a letra!");
                        chances -= 1;
                    }
                } else {
                    println!("Você já chutou essa letra!");
                }

            } else {
                if !checar_palavra(&palpite, &palavra) {
                    println!("Você errou a palavra!");
                    chances -= 1;
                }
                else {
                    println!("Para Bens você ganhou!");
                    break;
                }
            }

            if chances > 1 {
                println!("Restam {} chances", chances);
            } else {
                println!("Resta uma chance");
            }

            mostrar_palpites_anteriores_tela(&palpites_anteriores);

            mostrar_acertos_tela(&mut acertos);
        }
    }
}
