use std::{path::{Path, PathBuf}, str::FromStr};
use crate::{trashcan_config, misc_util};
use std::process::Command;
use chrono::Local;
use glob::glob;
use std::io;




pub fn clean_trashcan()
    {
    // invoke rm to empty trashcan and info folders
    let trashcan_str = trashcan_config::get_trashcan_location();
    let trashcan_path = Path::new(&trashcan_str);

    let info_file_str = trashcan_config::get_info_file_path();
    let info_file_path = Path::new(&info_file_str);

    // ask for user confirmation
    let mut input = String::new();
    println!("this will empty the following folders:");
    println!("\t{:?}", trashcan_path);
    println!("\t{:?}", info_file_path);
    println!("this will empty your trashcan, are you sure? Y/n");
    io::stdin().read_line(&mut input).unwrap();

    if input.trim() != "Y"
        {
        std::process::exit(0);
        }

    let status = Command::new("rm")
        .arg("-rf")
        .arg(trashcan_path)
        .output();

    status.expect("unable to empty trashcan folder, check permissions");

    let status = Command::new("rm")
        .arg("-f")
        .arg(info_file_path)
        .output();

    status.expect("unable to empty trashcan info file, check permissions");

    // create the trashcan and info file again
    let status = Command::new("mkdir")
        .arg("-p")
        .arg(trashcan_path)
        .output();

    status.expect("unable to create trashcan folder, check permissions");

    let status = Command::new("touch")
        .arg(info_file_path)
        .output();

    status.expect("unable to create trashcan info file, check permissions");

    }

pub fn move_file_to_trash(file_to_be_trashed: PathBuf)
    {
    let has_wildcards = check_wildcard_patterns(&file_to_be_trashed);
    match has_wildcards
        {
        Some(p) => 
            {
            eprintln!("{:?} contains a wildcard pattern", p);
            eprintln!("use the -r flag to enable wildcard and globbing support");
            std::process::exit(1);
            },
        None => {},
        }

    let trashcan_str = trashcan_config::get_trashcan_location();
    let trashcan_path = Path::new(&trashcan_str);

    let status = Command::new("mv")
        .arg("-f")
        .arg(&file_to_be_trashed.to_owned())
        .arg(trashcan_path)
        .output();

    /*
    [Trash Info]
    Path=/home/home/Desktop/Projects/BombTheCardGame/counter
    DeletionDate=2023-02-21T20:44:34
    */
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
                        add_info_file_to_trashcan(&file_to_be_trashed);
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

fn check_dangerous_patterns(pattern: &PathBuf) -> Option<&PathBuf>
    {
    let pts_msg = "unable to conver path buffer to string slice";
    let pbuf_str = pattern.to_str().expect(pts_msg);

    return match pbuf_str
        {
        "/" =>
            { 
            Some(pattern)
            },
        "~" =>
            {
            Some(pattern)
            },
        "~/" =>
            {
            Some(pattern)
            },
        "*" =>
            {
            Some(pattern)
            },
        "?" =>
            {
            Some(pattern)
            },
        _ =>
            { 
            None
            },
        }
    }

fn check_wildcard_patterns(pattern: &PathBuf) -> Option<&PathBuf>
    {
    let pts_msg = "unable to conver path buffer to string slice";
    let pbuf_str = pattern.to_str().expect(pts_msg);

    let wildcard_patterns = ['?', '*'];

    for i in pbuf_str.chars()
        {
        if (i == wildcard_patterns[0]) || (i == wildcard_patterns[1])
            {
            return Some(pattern);
            }
        }
    None
    }

pub fn move_pattern_to_trash(pattern: &str)
    {
    let binding = PathBuf::from(pattern);
    let has_dangerous_patterns = check_dangerous_patterns(&binding);

    match has_dangerous_patterns
        {
        Some(dpat) =>
            {
            match dpat.to_str().unwrap()
                {
                "/" =>
                    { 
                    let mut input = String::new();
                    println!("this will delete your entire root directory, are you sure? Y/n");
                    io::stdin().read_line(&mut input).unwrap();
                    if input.trim() != "Y"
                        {
                        std::process::exit(0);
                        }
                    },
                "~" =>
                    {
                    let mut input = String::new();
                    println!("this will delete your entire home directory, are you sure? Y/n");
                    io::stdin().read_line(&mut input).unwrap();
                    if input.trim() != "Y"
                        {
                        std::process::exit(0);
                        }
                    },
                "~/" =>
                    {
                    let mut input = String::new();
                    println!("this will delete your entire home directory, are you sure? Y/n");
                    io::stdin().read_line(&mut input).unwrap();
                    if input.trim() != "Y"
                        {
                        std::process::exit(0);
                        }
                    },
                "*" =>
                    {
                    let mut input = String::new();
                    println!("this will delete everything in your current directory, are you sure? Y/n");
                    io::stdin().read_line(&mut input).unwrap();
                    if input.trim() != "Y"
                        {
                        std::process::exit(0);
                        }
                    },
                _ => {   },
                    
                }
            }
        None => {},
        }

    for entry in glob(pattern).expect("unable to resolve globular pattern")
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
                                            add_info_file_to_trashcan(&PathBuf::from_str(err_file_name).unwrap());
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
                        eprintln!("move to trash failed even after a rename"); // panics because mv failed the second time here
                        std::process::exit(1);
                        }
                    }
                Err(_) =>
                    {
                    eprintln!("unable to move {:?}", err_file_name);
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

/// info files look like this
/// [Trash Info]
/// Path=/home/home/Desktop/math223textbook.pdf
/// DeletionDate=2023-09-11T00:59:06
fn add_info_file_to_trashcan(file_name: &PathBuf) 
    {
    let mut recov_info = misc_util::TrashInfo::new(file_name);
    recov_info.write(trashcan_config::get_info_file_path());
    }
