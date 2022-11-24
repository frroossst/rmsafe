use std::{process::Command, path::Path};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args
    {
    /// the file to be removed
    cmd: String,
    /// wildcard expressions matching 
    rgex: Option<String>,
    }

fn main() 
    {
    let args = Args::parse();

    let trashcan_path = Path::new("/home/home/.local/share/Trash/files");

    println!("saferm: powered with <3 by Rust");

    let cmd = Command::new("mv")
        .arg("./".to_string() + &args.cmd.to_owned())
        .arg(trashcan_path)
        .spawn();

    match cmd
        {
        Ok(_) => { cmd.unwrap(); },
        Err(e) => { println!("{:?}", e); }
        }
    }