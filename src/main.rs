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
    let trashcan_path = Path::new("/home/home/.local/share/Trash/files");
    let srm_trash = "/home/home/Desktop/.saferm_trash";
    // /home/home/.local/share/Trash/files/
    /*
    let mut cmd = String::new();
    cmd.push_str(cmd_prefix);
    cmd.push_str(&args.cmd);
    cmd.push_str(cmd_suffix);
    */

    // println!("{:?}", cmd);

    let pwd_vec = Command::new("pwd").output().unwrap().stdout;
    let pwd_path = String::from_utf8(pwd_vec[0..pwd_vec.len()-1].to_vec()).unwrap() + "/";

    let mut cmd = Command::new("mv");
    cmd.arg("./".to_string() + &args.cmd.to_owned());
    cmd.arg(trashcan_path);
    cmd.spawn().unwrap();

    println!("{:?}", cmd);
    }