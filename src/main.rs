use std::{io::{Write, Read}, fs::{File, OpenOptions}};

mod extensions;
mod warp;
mod invalid;

use extensions::Extensions;
use invalid::Invalid;

fn has_subtitles_extension(file_name: &str) -> bool {
    let ext = &file_name[file_name.rfind('.').unwrap() + 1..];
    Extensions::get().contains(&ext)
}

fn get_file_from_directory(first_arg: &str) -> Result<std::path::PathBuf, Invalid> {
    let entries_under = std::fs::read_dir(first_arg).map_err(|_| Invalid::FileSystemError)?;
    for entry in entries_under {
        if let Ok(e) = entry {
            if let Ok(file) = e.metadata() {
                if file.is_file() && has_subtitles_extension(e.file_name().to_str().unwrap()) {
                    return Ok(std::env::current_dir().unwrap().join(e.file_name().to_str().unwrap()));
                }
            }
        }
    }
    Err(Invalid::NoFileFound)
}

fn open_file(arg: &str) -> Result<std::fs::File, Invalid> {
    OpenOptions::new()
        .read(true)
        .write(true)
        .open(arg).map_err(|_| Invalid::CannotOpenFile)
}

fn get_file_path(first_arg: &str) -> Result<String, Invalid> {
    if first_arg == "." {
        return match get_file_from_directory(first_arg) {
            Ok(path) => Ok(path.to_str().unwrap().to_string()),
            Err(e) => Err(e)
        }
    }

    if let Ok(data ) = std::fs::metadata(first_arg) {
        if data.is_dir() {
            return match get_file_from_directory(first_arg) {
                Ok(path) => Ok(path.to_str().unwrap().to_string()),
                Err(e) => Err(e)
            }
        } else if data.is_file() {
            return Ok(first_arg.to_string());
        } else {
            return Err(Invalid::Args(first_arg.to_string()));
        }
    } 
    Err(Invalid::Args(first_arg.to_string()))
}

fn replace_text(file: &mut File, to_write: &Vec<u8>, path: &str) -> Result<(), Invalid> {
    drop(file);
    let mut file = File::create(path).unwrap();
    file.write_all(&to_write).map_err(|_| Invalid::CannotOpenFile)
}

fn warp_file(file: &mut File, change: &i64, path: &str) -> Result<(), Invalid> {
    let mut text: Vec<u8> = Vec::new();
    file.read_to_end(&mut text).or_else(|_| Err(Invalid::CannotOpenFile))?;
    warp::warp_text(&mut text, change);
    replace_text(file, &text, path)?;
    Ok(()) 
}

fn time_warp() -> Result<(), Invalid> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        return Err(Invalid::NoArg(2));
    }
    let file_arg = &args[1];
    let file = get_file_path(&file_arg)?;
    let mut file_handle = open_file(&file)?;
    let time_val: i64 = args[2].parse().map_err(|_| Invalid::Args(args[2].clone()))?;
    warp_file(&mut file_handle, &time_val, &file)
}

fn main() {
    if let Err(e) = time_warp() {
        e.alert_user_and_exit()
    }
}
