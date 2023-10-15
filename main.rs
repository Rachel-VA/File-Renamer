/*
Rachael Savage
SPS 323-Rust
Professor Tony Hinton
07/26/23
*/
//bring external crates (libraries) into the app. they must be added in the Cargo toml file under dependencies
//the word 'use' is to declare the usage so we don't have to write extra word every time we need to use it. ex: we use regex instead of regex::Regex
use ansi_term::Color; //to use colors for output text must add it in dependencies in Cargo.toml
use regex::Regex; //Regex struct from the regex crate/lib for pattern matching and manipulation file name
use std::fs; //standard lib to work with file system/directory including reading/writing/renaming files
use std::io;
use std::path::PathBuf; //standard lib struct for working w/ filesystem including building, manipulating, and directory paths
                        //the command line below for copy and paste the path to change name
                        //--------------------- cargo run -- "C:\\Users\\13039\\Pictures\\picture4Test" "^IMG" "Virginia" --------------------------

fn main() {
    // user multi-line for the Introduction. make to include the two {} {} for both the title and the paragraph because we change 2 different colors
    let introduction = format!(
        "{}\n{}",
        Color::Green.paint("\n\n\n========================= WELCOME TO THE FILE RENAMER APP =================================\n\n"),
        Color::Yellow.paint("This user-friendly app enables you to easily mass rename files/images and return them with numerical order in a directory.\n\
                            Whether you wish to rename files based on specific patterns or criteria,\n\
                            the File Renamer app is your go-to tool for effortless managing file names.\n\
                            \n\n\
                            INSTRUCTIONS:\n\
                            1. Enter the path to the folder containing the files you want to rename.\n\
                            2. Input the regular expression pattern to match the section of filenames you wish to modify.\n\
                            3. Provide the new name or regular expression pattern for renaming the files.\n\
                            \n\
                            \n\n\
                            EXAMPLE:\n\
                            To rename files in the folder 'C:\\Users\\Username\\Pictures\\Test' that begin with 'IMG_' and replace\n\
                            it with 'Seatle_', follow the prompts.\n\\
                            \n\
                            \n\
                            Happy file renaming!\n")
    );

    println!("{}", introduction);

    loop {
        
        // Get the directory path from the user
        println!("{}", Color::Cyan.paint("Enter the path to the folder containing the files you want to rename:"));
        let mut dir_path_input = String::new();//mutable var to store user input-path
        io::stdin()
        //          read user input and stores it in the dir_path_input var. &mut means that dir_path_input is mutable, it can be modified with the user input
            .read_line(&mut dir_path_input)
            .expect("Failed to read input");
        let dir_path = dir_path_input.trim();//create a new var & assign it to dir_path_input. trim() is use to remove whitespace from user input

        // Get the regular expression pattern from the user that will be used to match part of the filenames they want to rename
        println!("Enter the pattern name that you want to rename:");
        let mut pattern_input = String::new();//create a mut var string type
        io::stdin()//reads the user's input
            .read_line(&mut pattern_input)
            .expect("Failed to read input");//expect() is used to handle potential errors
        let pattern = pattern_input.trim();//create a var and assign to the value

        // Get the new name/pattern from the user to replace
        println!("{}", Color::Purple.paint("Enter the new name:"));
        let mut new_name_or_pattern_input = String::new();//create a new var string
        io::stdin()//read user input
            .read_line(&mut new_name_or_pattern_input)
            //.expect("Failed to read input");
            .expect(&Color::Red.paint("Failed to read input"). to_string());
        let new_name_or_pattern = new_name_or_pattern_input.trim();

        // Compile the regex pattern, if it's successful, it'll store the compiled regex pattern in the 're' var to be used later to match filenames
        let re = match Regex::new(pattern) {
            Ok(re) => re,
            Err(e) => {//use eprintln! macro to prints an error messag
                eprintln!(
                    "{} {}",
                    Color::Red.paint("Error: Invalid regex pattern:"),
                    e
                );
                continue;// used to skip the current iteration. no error, then move on
            }
        };

        // List all files in the directory. reads the specified directory path provided by the user using fs::read_dir
        let files = match fs::read_dir(dir_path) {//initiates a var files
            Ok(files) => files,//handle the result of the fs::read_dir(dir_path)
            Err(e) => {//handle errors
                eprintln!(
                    "{} {}",
                    Color::Red.paint("Error: Failed to read file directory:"),
                    e
                );
                continue;//skip the current iteration of the loop and proceed to the next iteration
            }
        };

        // Create a counter variable to keep track of the number of files renamed and give them numerical order
        let mut counter = 1;
        

        // Iterate through files and rename those that match the pattern. it handles the preparation step, extracting the file names, and storing them in a format suitable for renaming.
        for file in files {//initiates a var files to hold the list of files in the directory
            if let Ok(entry) = file {//check pattern matching is successful  from the file variable
                let file_name = entry.file_name();//retrieve the file name of the current file entry 
                let old_name = file_name.to_string_lossy().to_string();//s convert the file_name to a String

                if re.is_match(&old_name) {//check old name 
                    // Extract the file extension from the old name to keep the files/images extensions
                    let ext = match entry.path().extension().and_then(|e| e.to_str()) {
                        Some(ext) => ext.to_owned(),//valid extension that could be converted to a &str
                        None => {//has no extension or the extension could not be converted to a &str
                            eprintln!(
                                "{} {}",
                                Color::Red.paint("Error: Failed to get file extension."),
                                &old_name
                            );
                            continue;
                        }
                    };

                    // Generate the new name with counter and file extension
                    let new_name = format!("{}_{:04}.{}", new_name_or_pattern, counter, ext);

                    counter += 1; // Increment the counter
                    //renames files that match the regular expression pattern 
                    let new_path = PathBuf::from(dir_path).join(&new_name);//create a new var & concatenate the dir_path
                    //trying to rename the file using the fs::rename()
                    if let Err(e) = fs::rename(entry.path(), new_path) {
                        eprintln!("{} {}", Color::Red.paint("Error renaming file:"), e);// If the renaming process is successful, this line is executed
                    } else {
                        println!("Renamed: {} -> {}", old_name, new_name);//successfully renamed
                    }
                }
            }
        }

        println!("{}", Color::Fixed(211).paint("Files renamed successfully!"));//display a message

        // Ask the user if they want to rename more files
        println!("\nDo you want to rename more files? (y/n)\n");
        let mut answer = String::new();
        io::stdin()
            .read_line(&mut answer)
            .expect("Failed to read input");
        let answer = answer.trim().to_lowercase();
        //if user enter IS NOT y/yes, then break loop
        if answer != "y" && answer != "yes" {
            println!("{}", Color::Blue.paint("\nThank you for using the file_renamer app. Goodbye!\n")); //let the message display to user before the loop break out
            break; // Exit the loop if the user doesn't want to rename more files
        }
    } // End of the loop

    //println!("{}", Color::Fixed(211).paint("Files renamed successfully!"));
}
