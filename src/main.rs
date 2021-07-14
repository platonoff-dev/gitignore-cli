use clap::{App, Arg};
use reqwest;
use std::fs;
use dirs;

fn save_to_file() -> Result<(), reqwest::Error> {
    let mut response = reqwest::blocking::get("https://www.gitignore.io/api/list")?.text()?;
    response = response.replace(",", "\n");
    let mut save_path = dirs::home_dir().expect("Failed to gethome dir");
    save_path.push(".cache");
    save_path.push("gitignore_templates");
    fs::write(save_path.into_os_string(), response).expect("Save failed!!!");
    Ok(())
}

fn get_template_names() -> String {
    let mut save_path = dirs::home_dir().expect("Failed to gethome dir");
    save_path.push(".cache");
    save_path.push("gitignore_templates");

    match fs::read_to_string(save_path.into_os_string()) {
        Ok(templates) => templates,
        Err(_) => "".to_string(),
    }
}

fn main() {
    let cli_matches = App::new("Gitignore caching program")
        .version("0.1.0")
        .author("Platonov Anatoliy <p4m.dev@gmail.com>")
        .about("Can cache gitignore template names to ~/.cache folder")
        .arg(
            Arg::with_name("save")
                .short("s")
                .long("save")
                .takes_value(false)
                .help("Loads template names and save it into cache file"),
        )
        .get_matches();

    if cli_matches.is_present("save") {
        println!("--save found!!!");
        match save_to_file() {
            Ok(_) => println!("Result was ok!"),
            Err(_) => println!("Result was error!"),
        };
    } else {
        println!("{}", get_template_names())
    }
}
