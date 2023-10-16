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
    file_name: String,
    deletion_date: String,
    }

impl TrashInfo
    {
    pub fn new(path: &PathBuf) -> TrashInfo
        {
        let current_date = Utc::now();
        let deletion_date = current_date.format("%Y-%m-%dT%H:%M:%S").to_string();
        TrashInfo { file_name: path.clone().into_os_string().into_string().unwrap(), deletion_date }
        }

    pub fn write(&mut self, path: String) 
        {
        let mut content = String::from("[Trash Info]\n");

        content.push_str("Path=");
        content.push_str(&self.file_name);
        content.push_str("\n");

        content.push_str("DeletionDate=");
        content.push_str(&self.deletion_date);

        if self.file_name.ends_with("/")
            {
            self.file_name.pop();
            }

        let mut fobj = File::create(path + "/" + self.file_name.as_str()).unwrap();
        write!(fobj, "{}", content).unwrap();
        }
    }

pub fn display_settings()
    {
    println!("trashcan path: {:?}", trashcan_config::get_trashcan_location());
    println!("info file path: {:?}", trashcan_config::get_info_file_path());
    }

