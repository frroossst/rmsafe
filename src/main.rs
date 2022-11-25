use std::{process::Command, path::{Path, PathBuf}, io::Read};
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
    // let mut buf = String::new();
    // Command::new("whoami").spawn().unwrap().stdout.unwrap().read_to_string(&mut buf).unwrap();

    // let cstr_path = "/home/".to_owned() + &buf + "/.local/share/Trash/files";
    // println!("{:?}", cstr_path);
    let trashcan_path = Path::new("/home/home/.local/share/Trash/files");

    let mv_cmd = Command::new("mv")
        .arg(&file_to_be_trashed.to_owned())
        .arg(trashcan_path)
        .spawn();

    match mv_cmd
        {
        Ok(_) => { mv_cmd.unwrap(); },
        Err(e) => { println!("{:?}", e); }
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