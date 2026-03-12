use argon2::{
    password_hash::{PasswordHash, PasswordVerifier},
    Argon2
};

use rand_core::OsRng;

pub fn hash_password(password: &[u8]) -> Result<String, argon2::password_hash::Error> {
    // Generate a cryptographically secure salt
    let salt: SaltString = SaltString::generate(&mut OsRng);

    // Use recommended Argon2 configuration
    let argon2 = Argon2::default();

    // Hash password
    let password_hash = argon2.hash_password(password, &salt)?;

    // Return PHC formatted hash string
    Ok(password_hash.to_string())
}