// Import necessary standard library modules
use std::fs; // For file system operations
use std::error::Error; // To handle errors throughout the program
use clap::{Command, Arg}; // For parsing command line arguments

// The entry point of the program
fn main() {
    // Define the command line interface using `clap`
    // 'directory-printer' is the name of the program with a subcommand 'print-dir'
    let matches = Command::new("directory-printer")
        .version("1.0")
        .about("Prints the content of a directory")
        .subcommand(
            Command::new("print-dir")
                .about("Prints the content of a directory")
                .arg(Arg::new("dir") // Argument for the directory to print
                    .help("The directory to print")
                    .required(true)
                    .index(1))
        )
        .get_matches();

    // Check if the 'print-dir' subcommand was used
    if let Some(matches) = matches.subcommand_matches("print-dir") {
        // Get the 'dir' argument value, error if missing
        let dir = matches.get_one::<String>("dir").expect("Directory path is required");

        // Call the function to print the directory contents
        // Exit with an error code if the function returns an error
        if let Err(e) = print_dir_contents(dir) {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

// Function to print the contents of a directory
// Takes a directory path as input and returns a Result type
fn print_dir_contents(dir: &str) -> Result<(), Box<dyn Error>> {
    // Read the directory specified by 'dir' and handle any potential errors
    let paths = fs::read_dir(dir)?;

    // Iterate over the entries in the directory
    for path in paths {
        let path = path?; // Handle any errors in iteration
        // Print the file or directory name, converting any non-Unicode data to a lossy UTF-8 String
        println!("{}", path.file_name().to_string_lossy());
    }

    // Return Ok if everything went well
    Ok(())
}
