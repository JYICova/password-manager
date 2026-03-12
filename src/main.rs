use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};

fn main() {
    let password: &[u8; 8] = b"hunter42";
    let salt: SaltString = SaltString::generate(&mut OsRng);

    println!("{:?}", password);
    println!("{}", salt);

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

