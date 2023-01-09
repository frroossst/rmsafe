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

    match status
        {
        Ok(s) =>
            {
            let cmd_err_status = String::from_utf8(s.stderr);
            match cmd_err_status
                {
                Ok(conv) =>
                    {
                    if conv.trim().is_empty()
                        {
                        println!("removing {:?}", file_to_be_trashed);
                        }
                    else 
                        {
                        eprintln!("{:?}", conv);
                        eprintln!("attempting a rename and move on {:?}", file_to_be_trashed);
                        retry_move_with_file_rename(file_to_be_trashed)
                        }
                    }
                Err(_) =>
                    {
                    println!("unable to move {:?}", file_to_be_trashed);
                    }
                }
            }
        Err(_) =>
            {
            println!("unable to move {:?}", file_to_be_trashed);
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

    // this is just a rename
    let status = Command::new("mv")
        .arg("-f")
        .arg(&filename.to_owned())
        .arg(timestamp_name)
        .output();

    match status
        {
        Ok(s) =>
            {
            let cmd_err_status = String::from_utf8(s.stderr);
            match cmd_err_status
                {
                Ok(conv) =>
                    {
                    if conv.trim().is_empty()
                        {
                        println!("renaming {:?}", err_file_name);

                        let trashcan_str = trashcan_config::get_trashcan_location();
                        let trashcan_path = Path::new(&trashcan_str);

                        let status = Command::new("mv")
                            .arg("-f")
                            .arg(new_filename)
                            .arg(trashcan_path)
                            .output();

                        match status
                            {
                            Ok(s) =>
                                {
                                let cmd_err_status = String::from_utf8(s.stderr);
                                match cmd_err_status
                                    {
                                    Ok(conv) =>
                                        {
                                        if !conv.trim().is_empty()
                                            {
                                            eprintln!("{:?}", conv);
                                            }
                                        },
                                    Err(e) =>
                                        {
                                        eprintln!("{}", e);
                                        panic!("mv to trash failed after a rename {:?}", err_file_name);
                                        }
                                    }
                                },
                            Err(e) =>
                                {
                                eprintln!("{:?}", conv);
                                panic!("mv to trash failed even after a rename");
                                },
                            }

                        }
                    else 
                        {
                        eprintln!("{:?}", conv);
                        panic!("mv to trash failed even after a rename");
                        }
                    }
                Err(_) =>
                    {
                    eprintln!("unable to move {:?}", err_file_name);
                    }
                }
            }
        Err(_) =>
            {
            eprintln!("An error occurred removing {:?}", err_file_name);
            std::process::exit(1);
            }
        }

    }

fn concat_pathbuf_to_filename(file_path: PathBuf) -> PathBuf
    {
    let dt_obj = Local::now();
    let timestamp = dt_obj.timestamp();

    let timestamp_name = file_path.to_string_lossy().into_owned(); // + &timestamp.to_string();

    if (timestamp_name.chars().last().unwrap() == '/')
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
