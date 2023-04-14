use std::path::PathBuf;
use crate::trashcan_config;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};

/*
[Trash Info]
Path=/home/home/Desktop/Projects/BombTheCardGame/counter
DeletionDate=2023-02-21T20:44:34
*/
#[derive(Serialize)]
pub struct TrashInfo
    {
    path: String,
    deletion_date: String,
    }

impl TrashInfo
    {
    pub fn new(path: PathBuf) -> TrashInfo
        {
        let current_date = Utc::now();
        let deletion_date = current_date.format("%Y-%m-%dT%H:%M:%S").to_string();
        println!("deletion date: {:?}", deletion_date);
        TrashInfo { path: path.into_os_string().into_string().unwrap(), deletion_date }
        // write struct to file
        }
    }

pub fn display_settings()
    {
    println!("trashcan path: {:?}", trashcan_config::get_trashcan_location());
    println!("recovery file: {:?}", trashcan_config::get_recovery_location())
    }
