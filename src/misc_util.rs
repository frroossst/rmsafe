use crate::trashcan_config;

pub fn display_settings()
    {
    println!("trashcan path: {:?}", trashcan_config::get_trashcan_location());
    }