
//allow dead code
#![allow(dead_code)]
//allow unused variables
#![allow(unused_variables)]


//use rand crate
use rand::prelude::*;

mod database;
mod auth_utils;

pub use auth_utils::models::Credentials;
use database::Status;

pub fn authenticate(creds: Credentials){

    let timeout: i32 = thread_rng().gen_range(100..=500);

    println!("Timeout: {}", timeout);

    if let Status::Connected = database::connect_to_database() {
        auth_utils::login(creds);
    }

}