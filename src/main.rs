use std::io;

fn main() {
    println!("Hello, world!");
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(n) => {
            println!("{} bytes read", n);
            println!("{}", input);
        }
        Err(error) => println!("error: {}", error),
    }
    new_question("single_choice", &input);
}

fn new_question(question_type: &str, name: &str) {
    println!("{}", question_type);
    println!("{}", name);
}
