mod search;
mod user; 

pub fn main() {
    let user = user::file_input();
    println!("{}", user);
    let mut zips = search::for_zip_files(user);
    
   
}
