use std::io;
use argon2::password_hash;
use rusqlite::{Connection, Result};

pub fn get_user_input() -> String {
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");
    
    return user_input
}

pub fn add_user_to_table(connection: &Connection, username: &str, password_hash: &str) -> Result<()>{
    connection.execute(
        "INSERT INTO users (username, password_hash)
        VALUES (?1, ?2)
        ", 
        (username, password_hash),
    )?;
    Ok(())
}