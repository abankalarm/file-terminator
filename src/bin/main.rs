extern crate fileterminator;
extern crate dotext;
extern crate clap;
use clap::{Arg, App};
use fileterminator::*;

fn main() {
    let matches = App::new("file terminator")
                          .version("0.1")
                          .author("karan bamal bamalkaranbamal@gmail.com")
                          .about("reads docx file in terminal")
                          .arg(Arg::with_name("file")
                                .short("f")
                                .long("file")
                                .takes_value(true)
                                .help("A  file"))
                          .arg(Arg::with_name("docx")
                                .short("d")
                                .long("docx")
                                .takes_value(false)
                                .help("file type to open"))
                          .arg(Arg::with_name("pptx")
                                .short("p")
                                .long("pptx")
                                .takes_value(false)
                                .help("file type to open"))
                          .arg(Arg::with_name("v")
                               .short("v")
                               .multiple(true)
                               .help("Sets the level of verbosity"))
                          .get_matches();

    let path = matches.value_of("file").unwrap();
    let doc = matches.occurrences_of("docx");
    println!("{:?}", doc);
    if doc != 0{
        opendocx(path);
    }
    
        
}
