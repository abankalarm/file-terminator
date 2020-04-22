extern crate dotext;

use dotext::*;
use std::io::Read;


pub fn opendocx(path: &str)  {
    
    let mut file = Docx::open(path).expect("cannot open file");
    let mut isi = String::new();
    let _ = file.read_to_string(&mut isi);
    println!("CONTENT:");
    println!("--------BEGIN-------");
    println!("{}", isi);
    println!("--------END of file-------");

    
}

