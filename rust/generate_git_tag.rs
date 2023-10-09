use std::env;
use std::process::Command;

fn main(){
    let args: Vec<String> = env::args().collect();

    let mut repo = "opratel-core/platform-omanagerv2-front".to_string();
    let mut branch = "dev".to_string();

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--repo" => {
                i += 1;
                repo = args[i].clone();
            }
            "--branch" => {
                i += 1;
                branch = args[i].clone();
            }
            _ => {}
        }
        i += 1;
    }

    let output = Command::new("gh")
        .arg("release")
        .arg("list")
        .arg("--repo")
        .arg(&repo)
        .arg("--limit")
        .arg("1")
        .output()
        .expect("Failed to run gh command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let last_tag = stdout.split_whitespace().next().unwrap_or("");

    println!("The last version tag --> {}", last_tag);

    if last_tag.is_empty() {
        println!("No se encontró un tag anterior. Terminando el programa.");
        return;
    }

    let last_tag = &last_tag[1..]; // Remove the 'v'

    let parts: Vec<&str> = last_tag.split('.').collect();
    let major: i32 = parts[0].parse().unwrap_or(0);
    let minor: i32 = parts[1].parse().unwrap_or(0);
    let mut patch: i32 = parts[2].parse().unwrap_or(0);

    patch += 1;

    let new_tag = format!("v{}.{}.{}", major, minor, patch);

    Command::new("gh")
        .arg("release")
        .arg("create")
        .arg(&new_tag)
        .arg("-t")
        .arg(&new_tag)
        .arg("--notes")
        .arg(format!("Release {}", new_tag))
        .arg("--repo")
        .arg(&repo)
        .arg("--target")
        .arg(&branch)
        .spawn()
        .expect("Failed to run gh command");

    println!("New tag {} created for branch {}!", new_tag, branch);
}
