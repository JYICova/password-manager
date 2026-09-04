use rusqlite::{Connection, Result};
use std::{io::{self, Write}, println};
use crate::input::{add_user_to_table, get_user_input};
use crate::auth::hash_password;

#[derive(Debug, PartialEq, Eq)]
pub enum MenuChoice {
    Login,
    CreateAccount,
    Exit,
    Invalid,
}

pub fn create_account(connection: &Connection) -> Result<()> {
    // Enter Username
    println!("Enter your new username:");
    let username: String = get_user_input();

    println!("Enter your new password:");
    let password_text: String = get_user_input();
    let password_hash: String = hash_password(password_text.trim());

    // Validation
    // Create Profile
    add_user_to_table(connection, username.trim(), &password_hash);
    // Main Program
    Ok(())
}



pub fn initialise_table(connection: &Connection) -> Result<()> {

    let query = "
        CREATE TABLE IF NOT EXISTS users 
        (userID integer primary key autoincrement, 
        username TEXT NOT NULL UNIQUE, 
        passwordHash TEXTNOT NULL
        );
        -- INSERT INTO users VALUES ('Alice', 42);
        -- INSERT INTO users VALUES ('Bob', 69);
    ";
    connection.execute(query,[],)?;
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
    connection.query_row("SELECT EXISTS (
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
        Ok((
            row.get::<_, i64>(0)?,
            row.get::<_, String>(1)?,
        ))
    })?;

    for user in users {
        let (user_id, username) = user?;
        println!("user_id: {user_id}, username: {username}");
    }

    Ok(())
}
