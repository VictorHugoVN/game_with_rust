use std::io;
use std::cmp::Ordering;
use rand::Rng;


fn main(){
    println!("Adivinhe o número!");
    let numero_secreto = rand::thread_rng().gen_range(1..=20); //Definindo o intervalo do número secreto
    //println!("Número secreto: {}", numero_secreto);

    loop{

        let mut num_escolhido = String::new();

        println!("\nPor favor, insira seu palpite: ");

        io::stdin()
            .read_line(&mut num_escolhido)//Leitura do valor digitado pelo usuário
            .expect("Falha ao ler a linha!");

        let num_escolhido: u32 = match num_escolhido.trim().parse() {//Convertendo o tipo do número escolhido e tratando entrada inválida
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Seu palpite: {}", num_escolhido);

        match num_escolhido.cmp(&numero_secreto){//Comparação dos valores
            Ordering::Less => println!("seu palpite é menor que o número secreto!"),
            Ordering::Greater => println!("Seu palpite é maior que o número secreto!"),
            Ordering::Equal => {
                println!("Os valores são Iguais, você venceu!");
                break;
            }
        }
    }
    
}