use std::env;
use std::fs::{self, File};
use std::path::Path;
use std::io::{self, Write, BufRead};

fn main() -> io::Result<()> {
    // Definir los nombres del directorio y del archivo
    let dir = "logs";
    let file = "hub-opratel-dev.log";
    
    // Obtener la ruta completa del archivo de log
    let current_dir = env::current_dir()?;
    let full_path = current_dir.join(dir).join(file);
    
    // Crear el directorio si no existe
    if !full_path.parent().unwrap().exists() {
        fs::create_dir_all(full_path.parent().unwrap())?;
        println!("Directory {:?} created.", full_path.parent().unwrap());
    }
    
    // Crear el archivo de log si no existe
    if !full_path.exists() {
        File::create(&full_path)?;
        println!("File {:?} created.", full_path);
    }
    
    // Definir la ruta del archivo .env
    let env_file = current_dir.join(".env");
    
    // Leer el archivo .env, si existe, y almacenar las líneas en un vector
    let mut lines = if env_file.exists() {
        io::BufReader::new(File::open(&env_file)?).lines().collect::<Result<Vec<_>, _>>()?
    } else {
        Vec::new()
    };
    
    // Remover la variable de entorno antigua, si existe
    if let Some(pos) = lines.iter().position(|line| line.starts_with("LOGGUER_FILE_NAME=")) {
        lines.remove(pos);
        println!("Old environment variable LOGGUER_FILE_NAME deleted from .env file.");
    }
    
    // Añadir la nueva variable de entorno al vector
    lines.push(format!("LOGGUER_FILE_NAME={}", full_path.display()));
    println!("New environment variable LOGGUER_FILE_NAME added to .env file.");
    
    // Escribir las líneas actualizadas de nuevo al archivo .env
    fs::write(env_file, lines.join("\n"))?;
    
    Ok(())
}