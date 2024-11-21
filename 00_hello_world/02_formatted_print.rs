fn main() {
    println!("{} dias", 31);

    println!(
        "{0}, este é {1}. {1}, este é {0}",
        "João Marcelo", "Wladimir Tavares"
    );

    println!(
        "{sujeito} {verbo} {objeto}",
        objeto = "a medalha de ouro na olimpíada de programação.",
        sujeito = "O professor Wladimir",
        verbo = "conseguiu"
    );

    println!("Base 10:               {}", 69420); // 69420
    println!("Base 2 (binário):      {:b}", 69420); // 10000111100101100
    println!("Base 8 (octal):        {:o}", 69420); // 207454
    println!("Base 16 (hexadecimal): {:x}", 69420); // 10f2c

    println!("{numero:>5}", numero=1);
    println!("{numero:0>5}", numero=1); // 00001
    println!("{numero:0<5}", numero=1); // 10000
    println!("{numero:0>largura$}", numero=1, largura=5);
    
    //ERRO! Conserte!
    //println!("My name is {0}, {1} {0}", "Bond");

    #[allow(dead_code)] // desabilita warning em código não usado
    struct Structure(i32);
    //println!("This struct `{}` won't print...", Structure(3));
    
    let number: f64 = 1.0;
    let width: usize = 5;
    println!("Number: {number:>width$}");

    let pi:f64 = 3.141592;
    println!("O valor de PI é aproximadamente: {:.2}",pi);


}
