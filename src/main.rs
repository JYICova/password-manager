use std::println;

use crate::login::{
    create_account,
    get_menu_choice,
    initialise_table,
    show_users_table,
    table_exist,
    MenuChoice,
};




mod auth;
mod input;
mod login;
use rusqlite::{Connection, Result};

fn main() -> Result<()>{
    // Login or Create Account (only if at least 1 username exists)
    let table_name: &str = "users";
    let connection: Connection = Connection::open("app.db")?;

    if table_exist(&connection, table_name)? {
        println!("Loading...")
    } else {
        println!("Initialising Database...");
        initialise_table(&connection)?;
    }

    let menu_choice: MenuChoice = get_menu_choice();
    if menu_choice == MenuChoice::Login{
        // login()
    } else if menu_choice == MenuChoice::CreateAccount {
        create_account(&connection)?;
    } else if menu_choice == MenuChoice::Exit {
        // exit()
    } else {
        // try_again()
    };

            

    // Login (Selected)
    // Select Username
    // Enter Password

    // Create Account
    // Enter Username
    // Enter New Password
    
    show_users_table(&connection)?;
    Ok(())


    // let user_input: String = input::get_user_input();

    // let password: &str = user_input
    //                         .trim();

    // let hash: Result<String, argon2::password_hash::Error> = auth::hash_password(password);

    // println!("Password: {:?}", password);
    // println!("Hash: {:?}", hash);
}


