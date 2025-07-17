use std::env;
use std::error::Error;
use std::fmt;
use std::fs;
use std::io::{
    BufRead,
    BufReader,
};
use std::path::Path;
use std::process::Command;

struct Config {
    notes_directory: String,
    editor_command: String,
    config_file_path: String,
    extension: String,
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
    match args.len() {
        1 => help(), // good
        2 => match_singleton_command(&args[1], &config),
        3 => match_complex_command(&args, &config),
        _ => help(),
    }
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

    Ok(config_file)
}

fn unmarshall_yaml(config_file_path: &str) -> Result<Config, Box<dyn Error>> {
    let config_file = std::fs::File::open(config_file_path)?;

    let config_yaml: serde_yaml::Value = serde_yaml::from_reader(config_file)?;

    let config = Config {
        notes_directory: match config_yaml["notes_directory"].as_str() {
            Some(value) => value.to_owned(),
            None => String::new(),
        },
        editor_command: match config_yaml["editor_command"].as_str() {
            Some(value) => value.to_owned(),
            None => String::new(),
        },
        config_file_path: String::from(config_file_path),
        extension: match config_yaml["extension"].as_str() {
            Some(value) => value.to_owned(),
            None => String::from(".md"), // default to markdown
        },
    };

    Ok(config)
}

fn match_singleton_command(arg: &str, config: &Config) -> Result<(), Box<dyn Error>> {
    match arg {
        "help" => help(),
        "list" => list_notes(&config.notes_directory),
        "config" => edit_config(&config.config_file_path, &config.editor_command),
        _ => {
            let arg_string = String::from(arg);
            let file_name = if arg_string.contains(config.extension.as_str()) {
                arg_string
            } else {
                format!("{}{}", &arg_string, &config.extension)
            };

            create_or_find_note(&config.notes_directory, &config.editor_command, &file_name)
        }
    }
}

fn match_complex_command(args: &[String], config: &Config) -> Result<(), Box<dyn Error>> {
    let command: &str = &args[1][..];

    match command {
        "search" => search(&config.notes_directory, &String::from(&args[2])),
        "delete" => delete(
            &config.notes_directory,
            &format!("{}{}", &String::from(&args[2]), &config.extension),
        ),
        _ => help(),
    }
}

fn delete(notes_dir: &String, file_name: &String) -> Result<(), Box<dyn Error>> {
    let file_path = Path::new(notes_dir).join(file_name);
    fs::remove_file(&file_path)?;
    println!("{} deleted", file_path.display());
    Ok(())
}

fn edit_config(config_file_path: &String, editor_command: &String) -> Result<(), Box<dyn Error>> {
    let _output = Command::new(editor_command)
        .arg(config_file_path)
        .spawn()?
        .wait();

    Ok(())
}

fn create_or_find_note(
    notes_dir: &String,
    editor_command: &String,
    file_name: &String,
) -> Result<(), Box<dyn Error>> {
    let file_path = Path::new(notes_dir).join(file_name);
    let _output = Command::new(editor_command).arg(file_path).spawn()?.wait();

    Ok(())
}

// top level search, this searches for a search term in all available notes
// equivalent of cat/grep on the whole dir
fn search(notes_dir: &String, search_item: &String) -> Result<(), Box<dyn Error>> {
    let results: &mut Vec<FileSearchMatches> = &mut Vec::new();
    let paths = fs::read_dir(notes_dir)?;

    for path in paths {
        let dir_entry = path?;
        let path_string = dir_entry.path().into_os_string().into_string();
        match path_string {
            Ok(file_path) => {
                let search_result = search_file(&file_path, search_item)?;
                if search_result.matching_lines.is_empty() {
                    continue;
                } else {
                    results.push(search_result);
                }
            }
            Err(e) => {
                return Err(Box::new(MyError(format!(
                    "Failed to convert OsStr to String on item: {:?}",
                    e,
                ))))
            }
        }
    }

    if results.is_empty() {
        return Ok(());
    }

    for result in results {
        println!("{}", result.file_path);
        for line in &result.matching_lines {
            println!("{}", line);
        }
    }

    Ok(())
}

#[derive(Debug)]
struct FileSearchMatches {
    file_path: String,
    matching_lines: Vec<String>,
}

// returns filename and line with search term
// returns Result(FileSearchMatches, Error)
fn search_file(path: &String, search_item: &String) -> Result<FileSearchMatches, Box<dyn Error>> {
    let mut result = FileSearchMatches {
        file_path: path.to_string(),
        matching_lines: Vec::new(),
    };
    let file = fs::File::open(path).expect("File not found");
    let buf_reader = BufReader::new(file);

    for line_result in buf_reader.lines() {
        match line_result {
            Ok(line) => {
                if line.contains(search_item) {
                    result.matching_lines.push(line);
                }
            }
            Err(_) => continue,
        }
    }

    Ok(result)
}

fn list_notes(notes_dir: &str) -> Result<(), Box<dyn Error>> {
    let notes = get_notes(notes_dir)?;

    for note in notes {
        println!("{}   ", note);
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
    println!(
        "  list: list all existing notes in directory on separate lines (solely for grep purposes)"
    );
    println!("  search <value>: search contents of all notes for <value>, including filenames");
    println!("  config: edit config");
    println!("  delete: delete a note");

    Ok(())
}
