fn print_help_message() -> ! {
    eprintln!("Usage:");
    eprintln!("--force -f       force move files; typically just renames and moves");
    eprintln!("--help           prints this message");

    std::process::exit(1)
}

#[cfg(target_os = "linux")]
fn main() {
    let mut args = std::env::args();
    let _program = args.next();

    let arguments = args.collect::<Vec<String>>();

    let action = arguments.first().cloned();
    if let Some(cmd) = action {
        match cmd.as_str() {
            "--help" => print_help_message(),
            "-v" | "--version" => {
                eprintln!("{}", std::env!("CARGO_PKG_VERSION"));
                std::process::exit(0);
            },
            "-f" | "--force" => {
                let mut err_code = 0;
                let v = arguments.into_iter().skip(1).collect::<Vec<String>>();

                let timestamp = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .expect("[ERROR] SystemTime before UNIX EPOCH!").as_millis().to_string();

                for f in &v {
                    let cmd = std::process::Command::new("mv")
                        .arg(format!("{}_{}", f, timestamp))
                        .arg("/tmp/")
                        .status()
                        .expect("[ERROR] failed to execute the mv process");

                    if !cmd.success() {
                        eprintln!("[ERROR] failed to move file {:?} to /tmp/", f);
                        err_code = 1;
                    } else {
                        eprintln!("[OK] moved file {:?} to /tmp/", f);
                    }
                }

                std::process::exit(err_code);
            },
            _ => { // otherwise just move to /tmp/
                let mut err_code = 0;
                let v = arguments.into_iter().collect::<Vec<String>>();
                for f in &v {
                    let file_name = std::path::Path::new(f).file_name().unwrap_or_default().to_string_lossy().to_string();
                    let mut target_path = std::path::PathBuf::from("/tmp/");
                    target_path.push(&file_name);
                    // If file exists, append timestamp
                    if target_path.exists() {
                        let timestamp = std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .unwrap()
                            .as_secs();
                        let new_name = format!("{}_{}", file_name, timestamp);
                        target_path = std::path::PathBuf::from("/tmp/");
                        target_path.push(new_name);
                    }
                    let cmd = std::process::Command::new("mv")
                        .arg(f)
                        .arg(target_path.to_str().unwrap())
                        .status()
                        .expect("[ERROR] failed to execute the mv process");
                    if !cmd.success() {
                        eprintln!("[ERROR] failed to move file {:?} to /tmp/", f);
                        err_code = 1;
                    } else {
                        println!("[OK] moved file {:?} to {}", f, target_path.display());
                    }
                }
                std::process::exit(err_code);
            }
        }
    }
    print_help_message();
}
