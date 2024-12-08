use rand::Rng;
use std::io;

fn main() {
    
    let secret_number = gerar_numero_aleatorio();

    loop {
        let guess = match entrada_teclado() {
            Ok(num) => num,
            Err(error) => error,
        };
        if guess < 0 {
            println!("Digite um número válido!");
            continue;
        }
    
        match comparar_numeros(guess,secret_number) {
            Comparar::Maior => println!("O seu palpite é maior!"),
            Comparar::Menor => println!("O seu palpite é menor!"),
            Comparar::Igual => {
                println!("Parabéns! Você acertou!");
                break;
            }
        }; 
    }
    
}

/**
 * Gerar número aleatório
 */
fn gerar_numero_aleatorio() -> i32 {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("Número secreto gerado: {secret_number}");
    return secret_number;
}

/**
 * Retorna Result com o número ou com uma string de erro
 */
fn entrada_teclado() -> Result<i32, i32> {
    println!("Entre com o seu palpite: ");
    //lendo do teclado
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Falha ao ler linha");

    match guess.trim().parse() {
        Ok(num) => Ok(num),
        Err(_) => Err(-1),
    }
}

/**
 * Enum
 */
enum Comparar {
    Maior,
    Menor,
    Igual,
}

/**
 * Comparar os números
 */
fn comparar_numeros(guess: i32, secret_number: i32) -> Comparar {
    if guess > secret_number {
        Comparar::Maior
    } else if guess < secret_number {
        Comparar::Menor
    } else {
        Comparar::Igual
    }
}

/**
 * Versão 3 - Comparando o palpite com o número digitado
 */
fn main_3() {
    println!("**Adivinha o Número**");

    //gerando um número secreto de 1 a 100, incluindo estes
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("Número secreto gerado: {secret_number}");

    loop {
        println!("Entre com o seu palpite: ");
        //lendo do teclado
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Falha ao ler linha");

        //parse do número digitado para um inteiro de 32 bits (shadowing)
        //let guess : i32 = guess.trim().parse().expect("Digite um número válido!");

        //parse com tratamento (parse retorna um Result com duas invariantes, Ok e Err)
        //https://doc.rust-lang.org/std/result/
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Número inválido!");
                continue;
            }
        };

        if guess > secret_number {
            println!("Seu palpite é maior!");
        } else if guess < secret_number {
            println!("Seu palpite é menor");
        } else {
            println!("Parabéns! Você acertou!");
            break;
        }
    }
}

/**
 * Versão 2 - Geração de um número secreto!
 * Obs.: Não esqueça da dependência
 * [dependencies]
 * rand = "0.8.5"
 */
fn main_2() {
    //gerando um número secreto de 1 a 100, incluindo estes
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("Número secreto gerado: {secret_number}");
}

/**
 * Versão 1 - Lendo do teclado
*/
fn main_1() {
    println!("**Adivinha o Número**");
    println!("Entre com o seu palpite: ");

    let mut guess = String::new(); //criando um string vazia mutável
                                   //obs.: variáveis e referências em Rust são imutáveis por padrão!

    io::stdin()
        .read_line(&mut guess) //leitura do teclado, passando uma referência da variável guess (note que deve ser mut pois referência são imutáveis por padrão)
        .expect("Falha ao ler linha"); //caso a leitura apresente um erro

    println!("O seu palpite é {}", guess);
    println!("O seu palpite é {guess}");
}
