use std::path::PathBuf;
use crate::trashcan_config;
use std::fs::File;
use std::io::Write;
use chrono::prelude::*;

/*
[Trash Info]
Path=/home/home/Desktop/Projects/BombTheCardGame/counter
DeletionDate=2023-02-21T20:44:34
*/
pub struct TrashInfo
    {
    path: String,
    deletion_date: String,
    }

impl TrashInfo
    {
    pub fn new(path: &PathBuf) -> TrashInfo
        {
        let current_date = Utc::now();
        let deletion_date = current_date.format("%Y-%m-%dT%H:%M:%S").to_string();
        println!("deletion date: {:?}", deletion_date);
        TrashInfo { path: path.clone().into_os_string().into_string().unwrap(), deletion_date }
        }

    pub fn write(&self, path: String) 
        {
        let mut content = String::from("[Trash Info]\n");

        content.push_str("Path=");
        content.push_str(&self.path);
        content.push_str("\n");

        content.push_str("DeletionDate=");
        content.push_str(&self.deletion_date);

        let mut fobj = File::create(path).unwrap();
        write!(fobj, "{}", content).unwrap();
        }
    }

pub fn display_settings()
    {
    println!("trashcan path: {:?}", trashcan_config::get_trashcan_location());
    println!("info file path: {:?}", trashcan_config::get_info_file_path());
    }
