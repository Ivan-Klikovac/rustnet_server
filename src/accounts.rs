use std::fs::{File, OpenOptions};
use std::io::{self, prelude::*, BufReader};

pub struct User
{
    pub username: String,
    pub password: String,
}

impl User
{

}

pub fn create_account(username: &str, password: &str) -> io::Result<()>
{

}

pub fn login(username: &str, password: &str) -> io::Result<bool>
{
    
}