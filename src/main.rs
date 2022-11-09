use std::error::Error;
use serde_yaml;
use std::env;
use std::process::Command;
use std::fs;

struct Config {
    notes_directory: Box<String>,
    editor_command: Box<String>,
    config_file_path: Box<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    
    let config: Config = match unmarshall_yaml("/Users/cameron.kientz/.note-rs/config.yaml") {
        Ok(value) => value,
        Err(e) => return Err(e),
    };

    // args array includes executable itself
    let args: Vec<String> = env::args().collect();
    return match args.len() {
        1 => help(), // good
        2 => match_singleton_command(&args[1], &config),
        3 => match_complex_command(&args, &config),
        _ => help(),
    };
}

fn _match_input(files: Vec<String>, input: String) -> Result<Vec<String>, Box<dyn Error>> {
    let result: &mut Vec<String> = &mut Vec::new();

    for file in files {
        if file.starts_with(&input) {
            result.push(file);
        }
    }

    Ok(result.to_vec())
}


fn unmarshall_yaml(config_file_path: &str) -> Result<Config, Box<dyn Error>> {
    let config_file = match std::fs::File::open(config_file_path) {
        Ok(value) => value,
        Err(e) => return Err(Box::new(e)),
    };

    let config_yaml: serde_yaml::Value = match serde_yaml::from_reader(config_file) {
        Ok(value) => value,
        Err(e) => return Err(Box::new(e)),
    };

    let config = Config {
        notes_directory: match config_yaml["notes_directory"].as_str() {
            Some(value) => Box::new(value.to_owned()),
            None => Box::new(String::new()),
        },
        editor_command: match config_yaml["editor_command"].as_str() {
            Some(value) => Box::new(value.to_owned()),
            None => Box::new(String::new()),
        },
        config_file_path: Box::new(String::from(config_file_path)),
    };

    Ok(config)
}


fn match_singleton_command(arg: &str, config: &Config) -> Result<(), Box<dyn Error>> {
    return match arg {
        "help" => help(),
        "list" => list_notes(&config.notes_directory),
        "config" => edit_config(&config.config_file_path, &config.editor_command),
        _ => create_note(&config.notes_directory, &config.editor_command, &String::from(arg)),
    }
}

fn match_complex_command(args: &Vec<String>, config: &Config) -> Result<(), Box<dyn Error>> {
    let search_term: &String = &String::from(&args[2]);
    let command: &str = &args[1][..];

    return match command {
        "search" => search(&config.notes_directory, search_term),
        _ => help(),
    }
}

fn edit_config(config_file_path: &String, editor_command: &String) -> Result<(), Box<dyn Error>> {
    let _output = Command::new(editor_command)
        .arg(config_file_path)
        .spawn()?
        .wait();

    Ok(())
}

fn create_note(notes_dir: &String, editor_command: &String, file_name: &String) -> Result<(), Box<dyn Error>> {
    let file_path = format!("{}/{}.md", notes_dir, file_name);
    let _output = Command::new(editor_command)
        .arg(file_path)
        .spawn()?
        .wait();

    Ok(())
}

fn search(notes_dir: &String, search_item: &String) -> Result<(), Box<dyn Error>> {
    let output = match Command::new("rg")
        .arg(search_item)
        .arg(notes_dir)
        .output() {
            Ok(value) => value,
            Err(e) => return Err(Box::new(e)),
        };

    println!("{}", String::from_utf8_lossy(&output.stdout));
    Ok(())
}

fn list_notes(notes_dir: &String) -> Result<(), Box<dyn Error>> {
    let notes = match get_notes(notes_dir) {
        Ok(value) => value,
        Err(e) => return Err(e),
    };
    
    for note in notes {
        print!("{}   ", note);
    }
    
    Ok(())
}

fn get_notes(notes_dir: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let files = fs::read_dir(notes_dir).unwrap();
    let result: &mut Vec<String> = &mut Vec::new();

    for file in files {
        let file_name: String = file.unwrap().file_name().into_string().unwrap();
        result.push(file_name);
    }

    Ok(result.to_vec())
}

fn help() -> Result<(), Box<dyn Error>> {
    println!("Usage: note <name>: Create a new note in notes directory with name <name>");
    println!("  OR  ");
    println!("note <command> where <command> is one of:");
    println!("  list: list all existing notes in directory");
    println!("  search <value>: search contents of all notes for <value>, including filenames"); 
    println!("  config: edit config");

    Ok(())
}
