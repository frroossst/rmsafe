use rmsafe::{trashcan_config, remove_util, misc_util};
use love_rust::love_rust;
use std::path::PathBuf;
use clap::Parser;



#[derive(Parser, Debug)]
#[command(author, version)]
struct Args
    {
    /// Name of the singular file to be removed
    #[clap(value_parser, default_value="")]
    file: String,

    /// Wildcard expression matching pattern
    #[clap(short, long, value_parser)]
    rgex: Option<String>,

    /// Change trashcan path
    #[clap(short, long, value_parser)]
    trsh: Option<String>,

    /// Recovery file settings
    #[clap(short, long, value_parser)]
    vrec: Option<String>,
    }

fn main() 
    {
    let args = Args::parse();

    println!("{}", love_rust!("rmsafe"));

    let (mut flag_r, mut flag_t, mut flag_v) = (false, false, false);

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

    match args.vrec
        {
        Some(v) =>
            {
            trashcan_config::set_recovery_path(v);
            },
        None =>
            {
            flag_v = true;
            },
        }

    if !args.file.trim().is_empty()
        {
        remove_util::move_file_to_trash(PathBuf::from(args.file)); 
        }
     else 
        {
        if flag_r && flag_t && flag_v
            {
            misc_util::display_settings();
            }
        }

    }
