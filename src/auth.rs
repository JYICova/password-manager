use argon2::{
    Argon2, PasswordHasher, password_hash::{SaltString}
};
use argon2::password_hash::Error;

use rand::rngs::OsRng;

pub fn hash_password(password: &str) -> Result<String, Error> {
    // Generate a cryptographically secure salt
    let salt: SaltString = SaltString::generate(&mut OsRng);

    // Use recommended Argon2 configuration
    let argon2 = Argon2::default();

    // Hash password
    let password_hash = argon2.hash_password(password.as_bytes(), &salt)?;
    println!("password hash: {}", password_hash.to_string());
    // Return PHC formatted hash string
    Ok(password_hash.to_string())
}