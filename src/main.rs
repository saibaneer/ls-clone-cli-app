use std::fs;
use std::io;
use std::error::Error;
use clap::{Command, Arg};

fn main() {
    let matches = Command::new("directory-printer").version("1.0").about("Prints the content of a directory")
    .subcommand(Command::new("print-dir").about("Prints the content of a directory").arg(Arg::new("dir").help("The directory to print").required(true).index(1)))
    .get_matches();
    // print_dir_contents("/Users/gb3nga/Documents/").unwrap();
    // let dir = matches.get_one::<String>("dir").unwrap();
    // if let Err(e) = print_dir_contents(dir) {
    //     eprintln!("Error: {}", e);
    //     std::process::exit(1);
    // }

    if let Some(matches) = matches.subcommand_matches("print-dir"){
        let dir = matches.get_one::<String>("dir").expect("Directory path is required");
        if let Err(e) = print_dir_contents(dir) {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }

    
}


fn print_dir_contents(dir: &str) -> Result<(), Box<dyn Error>> {
    let paths = fs::read_dir(dir)?;

    for path in paths {
        let path = path?;
        println!("{}", path.file_name().to_string_lossy());
    }
    Ok(())
}
