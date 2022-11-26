use std::path::{Path, PathBuf};
use crate::trashcan_config;
use std::process::Command;
use glob::glob;



pub fn move_file_to_trash(file_to_be_trashed: PathBuf)
    {
    let trashcan_str = trashcan_config::get_trashcan_location();
    let trashcan_path = Path::new(&trashcan_str);

    let mv_cmd = Command::new("mv")
        .arg(&file_to_be_trashed.to_owned())
        .arg(trashcan_path)
        .spawn();

    match mv_cmd
        {
        Ok(_) => 
            { 
            mv_cmd.unwrap(); 
            },
        Err(e) => 
            { 
            println!("{:?}", e); 
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
            Err(e) => 
                { 
                println!("{:?}", e); 
                },
            }
        }
    }