use rmsafe::datetime::get_datetime;

fn print_help_message() -> ! {
    eprintln!("Usage:");
    eprintln!("--tmp -t         moves files to /tmp instead of trashcan");
    eprintln!("--reinit         resets config files to default");
    eprintln!("--restore        restores files that match the pattern");
    eprintln!("--help           prints this message");

    eprintln!("\nEdit ~/.config/rmsafe/config.toml to change default behaviour");

    let content_bytes =
        std::fs::read(Config::get_config_path()).expect("failed to read config file");
    let content = String::from_utf8(content_bytes).expect("config was not valid utf-8");

    eprintln!("\n{}", content.trim_end());

    std::process::exit(1)
}

struct Config {
    trashcan_location: String,
    recovery_location: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            trashcan_location: "/home/home/.local/share/Trash/files/".into(),
            recovery_location: "/home/home/.local/share/Trash/info/".into(),
        }
    }
}

impl std::fmt::Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "trashcan_location = {}\nrecovery_location = {}",
            self.trashcan_location, self.recovery_location
        )
    }
}

impl Config {
    fn get_config_path() -> std::path::PathBuf {
        let home = std::env::var("HOME").expect("no home tilde expansion found");
        std::path::PathBuf::from(format!("{}/.config/rmsafe/config.toml", home))
    }

    fn parse_config() -> Config {
        let config_path = Config::get_config_path();
        let content = std::fs::read_to_string(config_path).expect("unable to read config file");
        let lines = content.lines();

        let mut trashcan_location = Config::default().trashcan_location;
        let mut recovery_location = Config::default().recovery_location;

        for l in lines {
            if l.starts_with("trashcan_location") {
                let lsplt = l.split("=").collect::<Vec<&str>>();
                trashcan_location = lsplt
                    .last()
                    .expect("malformed config")
                    .to_string()
                    .trim()
                    .trim_matches(|c| c == '\'' || c == '"')
                    .to_string();
            } else if l.starts_with("recovery_location") {
                let lsplt = l.split("=").collect::<Vec<&str>>();
                recovery_location = lsplt
                    .last()
                    .expect("malformed config")
                    .to_string()
                    .trim()
                    .trim_matches(|c| c == '\'' || c == '"')
                    .to_string();
            }
        }

        Config {
            trashcan_location,
            recovery_location,
        }
    }

    fn ensure_config(self) {
        let config_path = Config::get_config_path();

        if let Some(parent) = config_path.parent() {
            std::fs::create_dir_all(parent).expect("unable to create config directory path");
        }

        if !config_path.exists() {
            let mut file =
                std::fs::File::create(&config_path).expect("unable to create config file");

            std::io::Write::write_all(&mut file, self.to_string().as_bytes())
                .expect("unable to write to config file");
        }

        let config: Config = Config::parse_config();

        let trashfiles = config.trashcan_location.clone();
        let infofiles = config.recovery_location.clone();

        std::fs::create_dir_all(std::path::Path::new(&trashfiles))
            .expect("unable to create trashcan directory");
        std::fs::create_dir_all(std::path::Path::new(&infofiles))
            .expect("unable to create recovery directory");
    }
}

fn remove_files(files: Vec<String>) {
    let config = Config::parse_config();

    for file in files {
        let fpath = std::path::Path::new(&file)
            .canonicalize()
            .map_err(|e| eprintln!("[ERROR] {:?} {:?}", &file, e))
            .expect("something went terribly wrong with path construction");

        let metadata = std::fs::symlink_metadata(&fpath)
            .map_err(|e| eprintln!("[ERROR] {:?} {:?}", &fpath, e))
            .expect("something went terribly wrong with path construction");
        if metadata.file_type().is_symlink() {
            eprintln!("[ERROR] refusing to remove symbolic link: {:?}", fpath);
            continue;
        }

        generate_info_file(&config, &fpath)
            .map_err(|e| eprintln!("[ERROR] {:?}", e))
            .ok();
        move_file_to_trashcan(&config, &fpath)
            .map_err(|e| eprintln!("[ERROR] {:?}", e))
            .ok();
    }
}

fn generate_info_file(
    config: &Config,
    file: &std::path::Path,
) -> std::result::Result<(), std::io::Error> {
    // make the info file at recovery_location/<filename>.trashinfo
    let filename = file.file_name().expect("unable to get filename from path");
    let mut where_to_write = std::path::PathBuf::from(&config.recovery_location);
    where_to_write.push(format!("{}.trashinfo", filename.to_string_lossy()));

    let content = format!(
        "[Trash Info]\nPath={:?}\nDeletionDate={}\n",
        file,
        get_datetime()
    );

    std::fs::write(where_to_write, content)
}

fn move_file_to_trashcan(
    config: &Config,
    file: &std::path::Path,
) -> std::result::Result<(), std::io::Error> {
    // move the file to trashcan_location/<filename>
    let filename = file.file_name().expect("unable to get filename from path");
    let mut where_to_move = std::path::PathBuf::from(&config.trashcan_location);
    where_to_move.push(filename);

    std::fs::rename(file, where_to_move)
}

#[cfg(target_os = "linux")]
fn main() {
    let mut args = std::env::args();
    let _program = args.next();

    Config::default().ensure_config();

    let arguments = args.collect::<Vec<String>>();

    let action = arguments.first().cloned();
    if let Some(cmd) = action {
        match cmd.as_str() {
            "--help" => print_help_message(),
            "-v" | "--version" => {
                eprintln!("{}", std::env!("CARGO_PKG_VERSION"));
                std::process::exit(0);
            },
            "-t" | "--tmp" => {
                let v = arguments.into_iter().skip(1).collect::<Vec<String>>();
                let files_to_remove = v.join(" ");

                let cmd = std::process::Command::new("mv")
                    .arg(files_to_remove)
                    .arg("/tmp/")
                    .status()
                    .expect("[ERROR] failed to execute the mv process");

                std::process::exit(cmd.code().unwrap_or(0));
            }
            "--reinit" => {
                let default_config = Config::default();
                let config_path = Config::get_config_path();

                std::fs::write(config_path, default_config.to_string())
                    .expect("unable to write default config");

                eprintln!("[OK]    config reset to default");

                std::process::exit(0);
            }
            "--restore" => {
                let c = Config::parse_config();

                let ls = std::fs::read_dir(std::path::Path::new(&c.recovery_location)).expect("oops");
                for i in ls {
                    dbg!(i.ok());
                }

                todo!()
            },
            _ => {
                // otherwise
                let v = arguments.into_iter().collect::<Vec<String>>();
                remove_files(v);

                std::process::exit(0);
            }
        }
    }
    print_help_message();
}
