use libreauth::pass::HashBuilder;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let password = &args[1];
    let hash_argon2 = HashBuilder::new().finalize().unwrap().hash(&password).unwrap();
    println!("{} -> {}",password, hash_argon2);
}
