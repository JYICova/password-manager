use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};

fn main() {
    println!("Hello, world!");
    let x = 5;
    println!("The value of x is: {x}");

    let x = x + 1;
    {
        let x = x + 5;
        println!("The value of x is: {x}");
    }
    println!("The value of x is: {x}");
}

