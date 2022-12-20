fn main() {
    let mut input = String::new();
    loop {
        match std::io::stdin().read_line(&mut input) {
            Ok(_) => {
                println!("You typed: {}", input);
            }
            Err(e) => {
                println!("{:?}", e);
            }
        }
        input.clear();
    }
}