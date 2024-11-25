//https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html
#[derive(Debug)]
struct Person <'a>{
    name: &'a str,
    age: u8,
} 

fn main() {
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    // Pretty print
    println!("{:#?}", peter);
}