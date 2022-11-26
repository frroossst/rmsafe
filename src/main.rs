use std::{process::Command, path::{Path, PathBuf}};
use clap::Parser;
use glob::glob;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args
    {
    /// Name of the singular file to be removed
    #[clap(short, long, value_parser)]
    file: Option<String>,

    /// Wildcard expression matching pattern
    #[clap(short, long, value_parser)]
    rgex: Option<String>,
    }
fn main() 
    {
    let args = Args::parse();

    println!("rmsafe: powered with <3 by Rust");

    match args.file
        {
        Some(c) => 
            { 
            move_file_to_trash(PathBuf::from(c)); 
            },
        None => 
            {   },
        }

    match args.rgex
        {
        Some(r) => 
            {
            move_pattern_to_trash(&r);
            },
        None =>
            {   },
        }

    }

fn move_file_to_trash(file_to_be_trashed: PathBuf)
    {
    // TODO: make username variable
    let mut usr_id = String::new();
    let whoami_cmd = Command::new("whoami").output(); //.read_to_string(&mut buf).unwrap();
    match whoami_cmd
        {
        Ok(w) =>
            {
            let whoami_str = String::from_utf8(w.stdout); //.replace("\n", ""));
            match whoami_str
                {
                Ok(s) =>
                    {
                    usr_id = s.replace("\n", "");
                    }
                Err(_) =>
                    {
                    println!("[ERROR] reading String from utf8 bytes");
                    }
                }
            }
        Err(e) =>
            {
            println!("whoami returns {:?}", e);
            }
        }

    let trashcan_path_prefix = "/home/";
    let trashcan_path_suffix = "/.local/share/Trash/files";
    let trashcan_str = trashcan_path_prefix.to_owned() + &usr_id + trashcan_path_suffix;
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

fn move_pattern_to_trash(pattern: &str)
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