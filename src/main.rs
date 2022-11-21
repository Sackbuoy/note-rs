use home;
use serde_yaml;
use std::env;
use std::error::Error;
use std::fmt;
use std::fs;
use std::process::Command;

struct Config {
    notes_directory: Box<String>,
    editor_command: Box<String>,
    config_file_path: Box<String>,
}

#[derive(Debug)]
struct MyError(String);

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for MyError {}

fn main() -> Result<(), Box<dyn Error>> {
    let config_file = find_config()?;

    let config: Config = unmarshall_yaml(&config_file)?;

    // args array includes executable itself
    let args: Vec<String> = env::args().collect();
    return match args.len() {
        1 => help(), // good
        2 => match_singleton_command(&args[1], &config),
        3 => match_complex_command(&args, &config),
        _ => help(),
    };
}

fn find_config() -> Result<String, Box<dyn Error>> {
    let home_path_buf = match home::home_dir() {
        Some(pathbuf) => pathbuf,
        None => return Err(Box::new(MyError("Could not get home directory".into()))),
    };

    let home = match home_path_buf.to_str() {
        Some(path) => path,
        None => return Err(Box::new(MyError("Could not get home directory".into()))),
    };

    let config_file = format!("{}/.note-rs/config.yaml", home);

    Ok(String::from(config_file))
}

fn unmarshall_yaml(config_file_path: &str) -> Result<Config, Box<dyn Error>> {
    let config_file = std::fs::File::open(config_file_path)?;

    let config_yaml: serde_yaml::Value = serde_yaml::from_reader(config_file)?;

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
        _ => create_note(
            &config.notes_directory,
            &config.editor_command,
            &String::from(arg),
        ),
    };
}

fn match_complex_command(args: &Vec<String>, config: &Config) -> Result<(), Box<dyn Error>> {
    let search_term: &String = &String::from(&args[2]);
    let command: &str = &args[1][..];

    return match command {
        "search" => search(&config.notes_directory, search_term),
        _ => help(),
    };
}

fn edit_config(config_file_path: &String, editor_command: &String) -> Result<(), Box<dyn Error>> {
    let _output = Command::new(editor_command)
        .arg(config_file_path)
        .spawn()?
        .wait();

    Ok(())
}

fn create_note(
    notes_dir: &String,
    editor_command: &String,
    file_name: &String,
) -> Result<(), Box<dyn Error>> {
    let file_path = format!("{}/{}.md", notes_dir, file_name);
    let _output = Command::new(editor_command).arg(file_path).spawn()?.wait();

    Ok(())
}

fn search(notes_dir: &String, search_item: &String) -> Result<(), Box<dyn Error>> {
    let output = Command::new("rg").arg(search_item).arg(notes_dir).output()?;

    println!("{}", String::from_utf8_lossy(&output.stdout));
    Ok(())
}

fn list_notes(notes_dir: &String) -> Result<(), Box<dyn Error>> {
    let notes = get_notes(notes_dir)?;

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
