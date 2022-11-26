use std::{process::Command, path::{Path, PathBuf}};
use serde_derive::{Serialize, Deserialize};
use clap::Parser;
use glob::glob;


#[derive(Debug, Serialize, Deserialize)]
struct RmsafeConfig
    {
    trashcan_location: String,
    }

impl std::default::Default for RmsafeConfig
    {
    fn default() -> Self 
        {
        let t = get_default_trashcan_location();
        Self { trashcan_location: String::from(t)  }
        }
    }

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

    /// Change trashcan path
    #[clap(short, long, value_parser)]
    trsh: Option<String>,

    /// Show trashcan path
    #[clap(default_value="")]
    show: String,
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

    match args.trsh
        {
        Some(t) =>
            {
            set_trashcan_path(t);
            },
        None =>
            {   },
        }

    match args.show
        {
        _ => 
            {
            display_settings()
            },
        }

    }

fn move_file_to_trash(file_to_be_trashed: PathBuf)
    {
    let trashcan_str = get_trashcan_location();
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

fn get_default_trashcan_location() -> String
    {
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

    trashcan_str
    }

fn get_trashcan_location() -> String
    {
    let cfg:RmsafeConfig = confy::load("rmsafe", None).unwrap();
    cfg.trashcan_location
    }

fn set_trashcan_path(t: String)
    {
    let mut cfg:RmsafeConfig = confy::load("rmsafe", None).unwrap();
    let old_path = cfg.trashcan_location;
    let new_path = String::from(t.clone());
    cfg.trashcan_location = t;
    let cnfy_cfg = confy::store("rmsafe", None, cfg);
    match cnfy_cfg
        {
        Ok(_) =>
            { 
            println!("trashcan location changed from {:?} to {:?}", old_path, new_path);
            }
        Err(e) =>
            {
            eprintln!("unable to write new trashcan path to config file");
            eprintln!("[ERROR] {:?}", e);
            }
        }
    }

fn display_settings()
    {
    println!("trashcan path: {:?}", get_trashcan_location());
    }