use serde_derive::{Serialize, Deserialize};
use std::{process::Command, fs, path::Path};



#[derive(Debug, Serialize, Deserialize)]
pub struct RmsafeConfig
    {
    pub trashcan_location: String,
    }

impl std::default::Default for RmsafeConfig
    {
    fn default() -> Self 
        {
        let t = get_default_trashcan_location();
        Self { trashcan_location: String::from(t)  }
        }
    }

pub fn get_trashcan_location() -> String
    {
    let cfg:RmsafeConfig = confy::load("rmsafe", None).unwrap();
    cfg.trashcan_location
    }

pub fn set_trashcan_path(t: String)
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
            let path = Path::new(&new_path);
            if !path.exists()
                {
                match fs::create_dir(path)
                    {
                    Ok(_) => { println!("created trashcan folder at path: {:?}", path); },
                    Err(e) => { eprintln!("[ERROR] {:?}", e) }
                    }
                }

            println!("trashcan location changed from {:?} to {:?}", old_path, new_path);
            }
        Err(e) =>
            {
            eprintln!("[ERROR] {:?}", e);
            }
        }
    }

fn get_default_trashcan_location() -> String
    {
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
                    let usr_id = s.replace("\n", "");
                    let trashcan_path_prefix = "/home/";
                    let trashcan_path_suffix = "/.local/share/Trash/files";
                    let trashcan_str = trashcan_path_prefix.to_owned() + &usr_id + trashcan_path_suffix;

                    trashcan_str
                    }
                Err(_) =>
                    {
                    std::process::exit(1);
                    }
                }
            }
        Err(_) =>
            {
            std::process::exit(1);
            }
        }
    }
