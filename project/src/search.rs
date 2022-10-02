use std::{fs,ffi::OsStr, path::{PathBuf,Path,self}};


pub fn for_zip_files(user_input: String) -> Vec<PathBuf>{
    let mut vec: Vec<PathBuf> = Vec::new(); 
    println!("{}", user_input);
    for file in fs::read_dir("/home/nb/Desktop/Testing/input/").unwrap(){ ///home/nb/Desktop/Testing/input/
        if file.as_ref().unwrap().path().extension().and_then(OsStr::to_str) == Some("zip"){
            println!("{} is a zip", file.as_ref().unwrap().path().display());
            vec.push(file.unwrap().path()); 
            
        }
        else {
            println!("{} is not a zip",file.as_ref().unwrap().path().display());
            
        }
        
    }
    if vec.len() != 0 {
        return vec;
    }   

    else {
        println!("no zip files found !");
        return vec;
    }

}


   


