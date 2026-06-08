mod auth;
use std::io;

fn main() {
    let user_input: String = get_user_input();
    println!("Entered Password: {}", user_input);

    let password: &[u8] = user_input.trim().as_bytes();
    // let salt: SaltString = SaltString::generate(&mut OsRng);

    let hash: Result<String, argon2::password_hash::Error> = auth::hash_password(password);

    println!("Password Binary: {:?}", password);
    println!("Hash: {:?}", hash);
}

fn get_user_input() -> String {
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");
    
    return user_input
}


