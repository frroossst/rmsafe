use rmsafe::{trashcan_config, remove_util, misc_util};
use std::path::PathBuf;
use clap::Parser;



#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args
    {
    /// Name of the singular file to be removed
    #[clap(value_parser, default_value="")]
    file: String,

    /// Wildcard expression matching pattern
    #[clap(short, long, value_parser)]
    rgex: Option<String>,

    /// Show trashcan path
    #[clap(short, long, value_parser)]
    trsh: Option<String>,

    /// Fail quietly
    #[clap(short, long, value_parser)]
    fail: Option<String>,
    }

fn main() 
    {
    let args = Args::parse();

    println!("rmsafe: powered with <3 by Rust");

    let (mut flag_r, mut flag_t) = (false, false);

    match args.rgex
        {
        Some(r) => 
            {
            remove_util::move_pattern_to_trash(&r);
            },
        None =>
            {   
            flag_r = true;
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
            flag_t = true;
            },
        }

    if !args.file.trim().is_empty()
        {
        remove_util::move_file_to_trash(PathBuf::from(args.file)); 
        }
     else 
        {
        if flag_r && flag_t
            {
            misc_util::display_settings();
            }
        }

    }
