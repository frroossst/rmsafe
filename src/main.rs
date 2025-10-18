fn print_help_message() -> ! {
    eprintln!("Usage:");
    eprintln!("--temp -t        moves files to /tmp instead of trashcan");
    eprintln!("--restore        restores files that match the pattern");
    eprintln!("--help           prints this message");

    eprintln!("Edit ~/.config/rmsafe/config.toml to change default behaviour");

    std::process::exit(1)
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
