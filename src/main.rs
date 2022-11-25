use std::{process::Command, path::{Path, PathBuf}};
use clap::Parser;
use glob::glob;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args
    {
    /// the file to be removed
    #[clap(short, long, value_parser)]
    cmd: Option<String>,

    /// wildcard expressions matching 
    #[clap(short, long, value_parser)]
    rgex: Option<String>,
    }

fn main() 
    {
    let args = Args::parse();

    println!("saferm: powered with <3 by Rust");

    // TODO: iterate and rm
    // let mut ls_cmd = Command::new("ls");
    // let ls_output = ls_cmd.spawn().unwrap();
    // let a = ls_output.stdout;
    // println!("{:?}", a);

    for entry in glob(&args.rgex.unwrap()).unwrap()
        {
        match entry
            {
            Ok(p) => 
                { 
                println!("{:?}", p.display()); 
                move_to_trash(p);
                },
            Err(e) => { println!("{:?}", e); },
            }
        }


    }

fn move_to_trash(file_to_be_trashed: PathBuf)
    {
    // TODO: make username variable
    let trashcan_path = Path::new("/home/home/.local/share/Trash/files");

    let mv_cmd = Command::new("mv")
        .arg(&file_to_be_trashed.to_owned())
        .arg(trashcan_path)
        .spawn();

    match mv_cmd
        {
        Ok(_) => { mv_cmd.unwrap(); },
        Err(e) => { eprintln!("{:?}", e); }
        }
    }