use serde_derive::{Serialize, Deserialize};
use std::{fs, path::Path};



#[derive(Debug, Serialize, Deserialize)]
pub struct RmsafeConfig
    {
    pub trashcan_location: String,
    pub recovery_location: String,
    }

impl std::default::Default for RmsafeConfig
    {
    fn default() -> Self 
        {
        let t = get_default_trashcan_location();
        let r = get_default_info_file_path();
            Self {
                trashcan_location: String::from(t),
                recovery_location: String::from(r),
            }
        }
    }

pub fn set_info_file_path(i: String)
    {
    let mut cfg:RmsafeConfig = confy::load("rmsafe", None).unwrap();

    let old_path = cfg.recovery_location;
    let new_path = String::from(i.clone());

    cfg.recovery_location = i;
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

pub fn get_info_file_path() -> String
    {
    let cfg:RmsafeConfig = confy::load("rmsafe", None).unwrap();
    cfg.recovery_location
    }

pub fn get_default_info_file_path() -> String
    {
    unimplemented!()
    }

pub fn get_default_trashcan_location() -> String
    {
    unimplemented!()
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

