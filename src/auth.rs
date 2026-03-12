use argon2::{
    password_hash::{PasswordHash, PasswordVerifier},
    Argon2
};

use rand_core::OsRng;

pub fn hash_password(password: &[u8]) -> Result<String, argon2::password_hash::Error> {

}