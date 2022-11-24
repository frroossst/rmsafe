use std::{process::Command, path::Path};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args
    {
    /// the rm command to be executed
    cmd: String,
    /// regular expressions matching 
    rgex: Option<String>,
    }

fn main() 
    {
    println!("saferm");
    println!("powered with <3 by Rust");

    let args = Args::parse();

    let cmd_prefix = "mv ";
    let trashcan_path = Path::new("/home/home/.local/share/Trash/files/");

    let mut cmd = Command::new(cmd_prefix);
    cmd.arg("./".to_string() + &args.cmd.to_owned());
    cmd.arg(trashcan_path);
    let result = cmd.spawn();
    result.unwrap();

    /* 
    match result
        {
        Ok(_) => { result.unwrap(); },
        Err(e) => { println!("{:?}", e) }
        }
    */

    }