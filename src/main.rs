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

    /// Change info file path
    #[clap(short, long, value_parser)]
    info: Option<String>,
    }

/// mv and rm commmands are supported on Linux only
fn check_compilation_target() 
    {
    #[cfg(not(target_os = "linux"))]
    compile_error!("This project is only supported on Linux.");
    }

fn main() 
    {
    check_compilation_target();

    let args = Args::parse();

    println!("{}", love_rust!("rmsafe"));

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

    match args.info
        {
        Some(i) =>
            {
            trashcan_config::set_info_file_path(i);
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
