use std::{fs,ffi::OsStr, path::{PathBuf, Path}};


//finding zip files 
pub fn checking_files(){
    //let mut i = 1;
    let mut vec: Vec<PathBuf> = Vec::new(); 
   
    for file in fs::read_dir("/home/nb/Desktop/Testing/input/").unwrap(){
        if file.as_ref().unwrap().path().extension().and_then(OsStr::to_str) == Some("zip"){
            println!("{} is a zip", file.as_ref().unwrap().path().display());
            vec.push(file.unwrap().path()); 
            
        }
        else {
            println!("{} is not a zip",file.as_ref().unwrap().path().display());
            
        }
        
    }
    println!("I found {} zip files", vec.len());
    println!(""); 

    for zips in vec {
        println!("{}", zips.display())
    }
}
   


