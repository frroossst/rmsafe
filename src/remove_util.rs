use std::path::{Path, PathBuf};
use crate::trashcan_config;
use std::process::Command;
use chrono::Local;
use glob::glob;



pub fn move_file_to_trash(file_to_be_trashed: PathBuf)
    {
    let trashcan_str = trashcan_config::get_trashcan_location();
    let trashcan_path = Path::new(&trashcan_str);

    let status = Command::new("mv")
        .arg("-f")
        .arg(&file_to_be_trashed.to_owned())
        .arg(trashcan_path)
        .output();

    match status // checks if the command failed
        {
        Ok(s) =>
            {
            let cmd_err_status = String::from_utf8(s.stderr);
            match cmd_err_status // checks if stderr occurred
                {
                Ok(conv) =>
                    {
                    if conv.trim().is_empty() // command succeeded and nothing is on stderr
                        {
                        println!("removing {:?}", file_to_be_trashed);
                        }
                    else // stderr has a string and calling on retry_move_with_file_rename()
                        {
                        eprintln!("{:?}", conv.trim());
                        eprintln!("attempting a rename and move on {:?}", file_to_be_trashed);
                        retry_move_with_file_rename(file_to_be_trashed)
                        }
                    }
                Err(_) =>
                    {
                    eprintln!("unable to convert stderr stream to UTF8 string");
                    std::process::exit(1);
                    }
                }
            }
        Err(e) =>
            {
            eprintln!("command failed: {:?}", e);
            std::process::exit(1);
            }
        }

    }

pub fn move_pattern_to_trash(pattern: &str)
    {
    for entry in glob(pattern).unwrap()
        {
        match entry
            {
            Ok(p) => 
                { 
                println!("removing: "); 
                print!("{:?}, ", p.display()); 
                move_file_to_trash(p);
                },
            Err(_) => 
                { 
                std::process::exit(1);
                },
            }
        }
    }

fn concat_pathbuf_to_filename(file_path: PathBuf) -> PathBuf
    {
    let dt_obj = Local::now();
    let timestamp = dt_obj.timestamp();

    let timestamp_name = file_path.to_string_lossy().into_owned(); // + &timestamp.to_string();

    if timestamp_name.chars().last().unwrap() == '/'
        {
        let length = timestamp_name.len();
        let converted_timestamp_name = timestamp_name.get(0..length - 1).unwrap().to_owned() + "_" + &timestamp.to_string() + "/";
        return PathBuf::from(converted_timestamp_name);
        }
    else
        {
        let converted_timestamp_name = timestamp_name.to_owned() + "_" + &timestamp.to_string();
        return PathBuf::from(converted_timestamp_name);
        }
    }

// If a similarly named folder exists in the
// trash folder then the mv command fails 
// with error, Directory not empty
// rmsafe changes the target file's name and
// then attempts the move, this is implemented
// because of common folder names like
// .git/ .cache/ etc.
pub fn retry_move_with_file_rename(filename: PathBuf)
    {
    let timestamp_name = concat_pathbuf_to_filename(filename.clone());
    let new_filename = timestamp_name.clone();
    let err_file_name = filename.to_str().unwrap();

    // this is just a rename to the same path
    let status = Command::new("mv")
        .arg("-f")
        .arg(&filename.to_owned())
        .arg(timestamp_name)
        .output();

    match status // check the status of the local move rename
        {
        Ok(s) =>
            {
            let cmd_err_status = String::from_utf8(s.stderr);
            match cmd_err_status // check if stderr has any string
                {
                Ok(conv) =>
                    {
                    if conv.trim().is_empty() // nothing on stderr
                        {
                        println!("renamed {:?} to {:?}", err_file_name, new_filename);

                        let trashcan_str = trashcan_config::get_trashcan_location();
                        let trashcan_path = Path::new(&trashcan_str);

                        let status = Command::new("mv")
                            .arg("-f")
                            .arg(new_filename)
                            .arg(trashcan_path)
                            .output();

                        match status // checking the status of the actual move of the rename to trahscan
                            {
                            Ok(s) =>
                                {
                                let cmd_err_status = String::from_utf8(s.stderr);
                                match cmd_err_status
                                    {
                                    Ok(conv) =>
                                        {
                                        if !conv.trim().is_empty() // do not recursively rename
                                            {
                                            eprintln!("rmsafe rename and move failed: {:?}", conv);
                                            }
                                        else
                                            {
                                            println!("removing {:?}", err_file_name);
                                            }
                                        },
                                    Err(_) =>
                                        {
                                        eprintln!("unable to convert stderr stream to UTF8 string");
                                        std::process::exit(1);
                                        }
                                    }
                                },
                            Err(e) =>
                                {
                                eprintln!("command failed: {:?}", e);
                                std::process::exit(1);
                                },
                            }

                        }
                    else 
                        {
                        eprintln!("couldn't rename file from {:?} to {:?}", err_file_name, new_filename);
                        panic!("move to trash failed even after a rename"); // panics because mv failed the second time here
                        }
                    }
                Err(_) =>
                    {
                    panic!("unable to move {:?}", err_file_name);
                    }
                }
            }
        Err(e) =>
            {
            eprintln!("command failed: {:?}", e);
            std::process::exit(1);
            }
        }

    }
