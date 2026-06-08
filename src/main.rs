mod auth;
mod input;

fn main() {
    // Login or Create Account (only if at least 1 username exists)

    // Login (Selected)
    // Select Username
    // Enter Password

    // Create Account
    // Enter Username
    // Enter New Password


    let user_input: String = input::get_user_input();

    let password: &str = user_input
                            .trim();

    let hash: Result<String, argon2::password_hash::Error> = auth::hash_password(password);

    println!("Password Binary: {:?}", password);
    println!("Hash: {:?}", hash);
}


