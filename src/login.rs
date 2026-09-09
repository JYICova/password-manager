use crate::auth::hash_password;
use crate::input::{add_user_to_table, get_user_input};
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use rusqlite::{Connection, Error as SqliteError, Result, named_params};
use std::{
    io::{self, Write},
    println,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("database error: {0}")]
    Database(#[from] SqliteError),

    #[error("password verification failed")]
    InvalidPassword,

    #[error("stored password hash is invalid")]
    InvalidStoredHash,
}

#[derive(Debug, PartialEq, Eq)]
pub enum MenuChoice {
    Login,
    CreateAccount,
    Exit,
    Invalid,
}
#[derive(Debug)]
pub struct User {
    user_id: i64,
    username: String,
    password_hash: String,
}

pub fn create_account(connection: &Connection) -> Result<()> {
    // Enter Username
    println!("Enter your new username:");
    let username: String = get_user_input();

    println!("Enter your new password:");
    let password_text: String = get_user_input();
    let password_hash: String =
        hash_password(password_text.trim()).expect("Failed to hash password");

    println!("Password Hash: {}", password_hash);
    // Validation
    // Create Profile
    add_user_to_table(connection, username.trim(), &password_hash.as_str())?;
    println!("{} username added", username);
    // Main Program
    Ok(())
}

pub fn initialise_table(connection: &Connection) -> Result<()> {
    let query = "
        CREATE TABLE IF NOT EXISTS users 
        (user_id integer primary key autoincrement, 
        username TEXT NOT NULL UNIQUE, 
        password_hash TEXT NOT NULL
        );
        -- INSERT INTO users VALUES ('Alice', 42);
        -- INSERT INTO users VALUES ('Bob', 69);
    ";
    connection.execute(query, [])?;
    Ok(())
}

pub fn get_menu_choice() -> MenuChoice {
    println!();
    println!("========================");
    println!("      Password Manager");
    println!("========================");
    println!();
    println!("1. Login");
    println!("2. Create account");
    println!("3. Exit");
    println!();
    print!("Select an option: ");

    io::stdout().flush().expect("Failed to flush stdout");
    let user_input: String = get_user_input();

    match user_input.trim() {
        "1" => MenuChoice::Login,
        "2" => MenuChoice::CreateAccount,
        "3" => MenuChoice::Exit,
        _ => MenuChoice::Invalid,
    }
}

pub fn table_exist(connection: &Connection, table_name: &str) -> Result<bool> {
    connection.query_row(
        "SELECT EXISTS (
                            SELECT 1 
                            FROM sqlite_master
                            WHERE type = 'table'
                            AND name = ?1
                            )",
        [table_name],
        |row| row.get(0),
    )
}

pub fn show_users_table(conn: &Connection) -> Result<()> {
    let mut statement = conn.prepare("SELECT * FROM users")?;

    let users = statement.query_map([], |row| {
        Ok((row.get::<_, i64>(0)?, row.get::<_, String>(1)?))
    })?;

    for user in users {
        let (user_id, username) = user?;
        println!("user_id: {user_id}, username: {username}");
    }

    Ok(())
}

// pub fn retrieve_

pub fn login(connection: &Connection) -> Result<User, AuthError> {
    // Enter Username
    println!("Enter your username:");
    let username: String = get_user_input();

    println!("Enter your password:");
    let password_text: String = get_user_input();
    let password_hash: String =
        hash_password(password_text.trim()).expect("Failed to hash password");

    let user_details: User = verify_password(connection, &username, &password_hash)?;

    Ok(user_details)
}

pub fn get_user_login_details(connection: &Connection, username: &str) -> Result<User> {
    connection.query_row(
        r#"
        SELECT user_id, username, password_hash
        FROM users
        WHERE username = :username
        "#,
        named_params! {
            ":username": username
        },
        |row| {
            Ok(User {
                user_id: row.get("user_id")?,
                username: row.get("username")?,
                password_hash: row.get("password_hash")?,
            })
        },
    )
}

pub fn verify_password(
    connection: &Connection,
    username: &String,
    password_hash: &String,
) -> Result<User, AuthError> {
    let retrieved_user_information: User = get_user_login_details(connection, username)?;
    let stored_password_hash: &str = retrieved_user_information.password_hash.as_str();

    let parsed_hash =
        PasswordHash::new(&stored_password_hash).map_err(|_| AuthError::InvalidStoredHash)?;

    Argon2::default()
        .verify_password(password_hash.as_bytes(), &parsed_hash)
        .map_err(|_| AuthError::InvalidPassword)?;
    Ok(retrieved_user_information)
}
