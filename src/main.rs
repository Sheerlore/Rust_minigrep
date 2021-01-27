use std::env;

fn main() {
    println!("minigrep project!");

    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    
}
