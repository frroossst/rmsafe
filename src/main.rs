use std::process::Command;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args
    {
    /// the rm command to be executed
    cmd: String,
    }

fn main() 
    {
    println!("saferm");
    println!("powered with <3 by Rust");

    let args = Args::parse();

    println!("{:?}", args.cmd);

    let cmd_prefix = "mv ";
    let cmd_suffix = " ~/.local/share/Trash/files";

    /*
    let mut cmd = String::new();
    cmd.push_str(cmd_prefix);
    cmd.push_str(&args.cmd);
    cmd.push_str(cmd_suffix);
    */

    // println!("{:?}", cmd);

    let pwd_path = Command::new("pwd").output().unwrap();


    Command::new(cmd_prefix).args(&[args.cmd, pwd_path.stdout.to_string() + &cmd_suffix.to_owned(), cmd_suffix.to_owned()]).output().expect("mv failed");
    }
