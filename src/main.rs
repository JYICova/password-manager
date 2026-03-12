mod auth;

fn main() {
    let password: &[u8; 8] = b"hunter42";
    // let salt: SaltString = SaltString::generate(&mut OsRng);

    let hash: Result<String, argon2::password_hash::Error> = auth::hash_password(password);

    println!("{:?}", password);
    println!("{:?}", hash);
}

