use std::env;
use std::fs::{self, File};
use std::path::Path;
use std::io::{self, Write, BufRead};

fn main() -> io::Result<()> {
    // Define directory and file names
    let dir = "logs";
    let file = "hub-opratel-dev.log";
    
    // Get the full path of the log file
    let current_dir = env::current_dir()?;
    let full_path = current_dir.join(dir).join(file);
    
    // Create the directory if it does not exist
    if !full_path.parent().unwrap().exists() {
        fs::create_dir_all(full_path.parent().unwrap())?;
        println!("Directory {:?} created.", full_path.parent().unwrap());
    }
    
    // Create the log file if it does not exist
    if !full_path.exists() {
        File::create(&full_path)?;
        println!("File {:?} created.", full_path);
    }
    
    // Define the path of the .env file
    let env_file = current_dir.join(".env");
    
    // Read the .env file, if it exists, and store the lines in a vector
    let mut lines = if env_file.exists() {
        io::BufReader::new(File::open(&env_file)?).lines().collect::<Result<Vec<_>, _>>()?
    } else {
        Vec::new()
    };
    
    // Remove the old environment variable, if it exists
    if let Some(pos) = lines.iter().position(|line| line.starts_with("LOGGUER_FILE_NAME=")) {
        lines.remove(pos);
        println!("Old environment variable LOGGUER_FILE_NAME deleted from .env file.");
    }
    
    // Add the new environment variable to the vector
    lines.push(format!("LOGGUER_FILE_NAME={}", full_path.display()));
    println!("New environment variable LOGGUER_FILE_NAME added to .env file.");
    
    // Write the updated lines back to the .env file
    fs::write(env_file, lines.join("\n"))?;
    
    Ok(())
}
