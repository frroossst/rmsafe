fn print_help_message() -> ! {
    eprintln!("Usage:");
    eprintln!("--temp -t        moves files to /tmp instead of trashcan");
    eprintln!("--restore        restores files that match the pattern");
    eprintln!("--help           prints this message");

    eprintln!("\nEdit ~/.config/rmsafe/config.toml to change default behaviour");


    let content_bytes = std::fs::read(get_config_path()).expect("failed to read config file");
    let content = String::from_utf8(content_bytes).expect("config was not valid utf-8");

    eprintln!("\n{}", content.trim_end());

    std::process::exit(1)
}

#[inline]
fn get_config_path() -> std::path::PathBuf {
    let home = std::env::var("HOME").expect("no home tilde expansion found");
    std::path::PathBuf::from(format!("{}/.config/rmsafe/config.toml", home))
}

#[inline]
fn ensure_config() {
    let config_path = get_config_path();

    if let Some(parent) = config_path.parent() {
        std::fs::create_dir_all(parent).expect("unable to create config directory path");
    }

    if !config_path.exists() {
        let mut file = std::fs::File::create(&config_path).expect("unable to create config file");

        std::io::Write::write_all(&mut file, 
            b"trashcan_location = '~/.local/share/Trash/files/'\nrecovery_location = '~/.local/share/Trash/info/'\n",
            ).expect("unable to write to config file");
    }

}

fn remove_files(files: Vec<String>) {

}

fn generate_info_file(file: String) {

}

fn move_file_to_trashcan(file: String) {

}

fn main() {
    let mut args = std::env::args();
    let _program = args.next();

    ensure_config();

    let arguments = args.collect::<Vec<String>>();

    let action = arguments.first().cloned();
    if let Some(cmd) = action {
        match cmd.as_str() {
            "-t" | "--tmp" => todo!("move to /tmp"),
            "--restore" => todo!("restore all things that match the glob"),
            _ => {
                let v = arguments.into_iter().collect::<Vec<String>>();
                remove_files(v);

                std::process::exit(0);
            }
        }
    } 

    print_help_message();
}
