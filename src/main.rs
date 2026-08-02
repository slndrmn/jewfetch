mod utils;

use std::fs;
use serde::{Serialize,Deserialize};
use utils::{sh, get_config_dir};

#[derive(Serialize,Deserialize,Debug)]
struct Config {
    color:String,
    ascii:String,
    components:Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            color:"\x1b[1;94m".to_string(),
            ascii:"default".to_string(),
            components: vec![
              "host".to_string(),
              "os".to_string(),
              "kernel".to_string(),
              "uptime".to_string(),
              "de".to_string(),
              "shell".to_string(),
              "mem".to_string()
            ],
        }
    }
}

fn take_config() -> Config {
    let config_path = get_config_dir().join("config.json");
    if let Ok(json_content) = fs::read_to_string(config_path) {
        if let Ok(result) = serde_json::from_str(&json_content) {
            return result;
        }
    }

    Config::default()
}

fn get_ascii_art() -> String {
    let config = take_config();
    let ascii_art = config.ascii;

    if let Ok(result) = fs::read_to_string(get_config_dir().join("ascii-arts").join(format!("{}.txt",ascii_art))) {
        result
    } else {
        format!("could'nt find the file: {}.txt",ascii_art).to_string()
    }
}

fn get_components() -> Vec<(String,String)>{
    let config = take_config();
    let mut parsed_components: Vec<(String,String)> = Vec::new();

    for component in config.components.iter() {
        let script_file = get_config_dir().join("commands").join(format!("{}",component));

        let command = if component.ends_with(".sh") {
            format!("bash {}", script_file.to_str().unwrap())
        } else if component.ends_with(".py") {
            format!("python {}", script_file.to_str().unwrap())
        } else {
            format!("./{}", script_file.to_str().unwrap())
        };

        let text = sh(&command);
        let sliced_text: Vec<&str> = text.split(':').collect();

        if sliced_text.len() == 1 {
            parsed_components.push(("".to_string(),sliced_text[0].trim().to_string()));
        } else {
            parsed_components.push((sliced_text[0].to_string(),sliced_text[1].trim().to_string()));
        }
    };
    parsed_components
}

fn main() {
    let ascii_art = get_ascii_art();
    let config = take_config();
    let reset_color = "\x1b[0m";
    let components = get_components();

    let color = match config.color.as_str() {
        "black" => "\x1b[1;90m",
        "red" => "\x1b[1;91m",
        "green" => "\x1b[1;92m",
        "yellow" => "\x1b[1;93m",
        "blue" => "\x1b[1;94m",
        "purple" => "\x1b[1;95m",
        "cyan" => "\x1b[1;96m",
        "white" => "\x1b[1;97m",
        _ => panic!("error while finding color (maybe config file doesnt exist??)"),
    };

    let mut max_line = 0;
    for line in ascii_art.lines() {
        let len = line.len();
        if len > max_line {
            max_line = len;
            continue;
        }
    }

    println!();
    let mut i = 0;
    for line in ascii_art.lines() {
        if i == components.len()+1 {
            println!("{}{}{}",color,line,reset_color);
        } else if i == 0 {
            println!("{}{:<width$}  {}", color, line, components[i].1, width = max_line);
            i += 1;
        } else if i == 1 {
            println!("{}{:<width$}{}  {}", color, line, reset_color,"-".repeat((components[0].1).chars().count()), width = max_line);
            i += 1;
        } else {
            println!("{}{:<width$}  {}{}: {}", color, line, &components[i-1].0, reset_color,components[i-1].1, width = max_line);
            i += 1;
        }
    }
    println!();
}
