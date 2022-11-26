use rmsafe::{trashcan_config, remove_util, misc_util};
use std::path::PathBuf;
use clap::Parser;



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

    /// Change or show trashcan path
    #[clap(short, long, value_parser)]
    trsh: Option<String>,
    }

fn main() 
    {
    let args = Args::parse();

    println!("rmsafe: powered with <3 by Rust");

    let mut flag: bool = false;

    match args.file
        {
        Some(f) =>
            {
            remove_util::move_file_to_trash(PathBuf::from(f)); 
            },
        None =>
            {   
            flag = true;
            },
        }

    match args.rgex
        {
        Some(r) => 
            {
            remove_util::move_pattern_to_trash(&r);
            },
        None =>
            {   
            flag = true;
            },
        }

    match args.trsh
        {
        Some(t) =>
            {
            trashcan_config::set_trashcan_path(t);
            },
        None =>
            {
            flag = true;
            },
        }

    if flag
        {
        misc_util::display_settings();
        }

    }
