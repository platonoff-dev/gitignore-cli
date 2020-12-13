use clap::{App, Arg};
use reqwest;
use std::fs;

fn save_to_file() -> Result<(), reqwest::Error> {
    let mut response = reqwest::blocking::get("https://www.gitignore.io/api/list")?.text()?;
    response = response.replace(",", "\n");
    fs::write("/home/anatoliy/.cache/gitignore_templates", response).expect("Save failed!!!");
    Ok(())
}

fn get_template_names() -> String {
    match fs::read_to_string("/home/anatoliy/.cache/gitignore_templates") {
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
