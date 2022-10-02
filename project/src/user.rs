use std::{io}; 

pub fn file_input() -> String{
    let mut strpath = String::new(); 
    io::stdin().read_line(&mut strpath);
    return strpath;
}