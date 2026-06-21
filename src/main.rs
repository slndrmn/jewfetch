mod utils;

use std::fs;
use serde::{Serialize,Deserialize};
use utils::{sh, get_config_dir};

#[derive(Serialize,Deserialize,Debug)]
struct Preferences {
    host:(String,String),
    os:(String,String),
    kernel:(String,String),
    uptime:(String,String),
    de:(String,String),
    shell:(String,String),
    mem:(String,String),
}

#[derive(Serialize,Deserialize,Debug)]
struct Options {
    color:String,
    ascii:String,
}

impl Default for Preferences {
    fn default() -> Self {
        Self {
            host:("".to_string(),"echo $USER@$(cat /etc/hostname)".to_string()),
            os:("os".to_string(),"echo \"$(grep '^PRETTY_NAME=' /etc/os-release | cut -d'\"' -f2) $(uname -m)\"".to_string()),
            kernel:("kernel".to_string(),"uname -s -r".to_string()),
            uptime:("uptime".to_string(),"uptime -p".to_string()),
            de:("de/wm".to_string(),"echo $XDG_CURRENT_DESKTOP $DESKTOP_SESSION".to_string()),
            shell:("shell".to_string(),"zsh --version | cut -d' ' -f1,2".to_string()),
            mem:("mem".to_string(),"free -m | grep Mem | awk '{print $3 \"MB / \" $2 \"MB\"}'".to_string()),
        }
    }
}

impl Default for Options {
    fn default() -> Self {
        Self {
            color:"\x1b[1;94m".to_string(),
            ascii:"default".to_string(),
        }
    }
}

impl Preferences {
    pub fn as_vector(&self) -> Vec<(String,String)> {
        Vec::from([(self.host.0.clone(),self.host.1.clone()),
            ("".to_string(),"".to_string()),
            (self.os.0.clone(),self.os.1.clone()),
            (self.kernel.0.clone(),self.kernel.1.clone()),
            (self.uptime.0.clone(),self.uptime.1.clone()),
            (self.de.0.clone(),self.de.1.clone()),
            (self.shell.0.clone(),self.shell.1.clone()),
            (self.mem.0.clone(),self.mem.1.clone()),])
    }
}

fn take_config() -> Preferences {
    let config_path = get_config_dir().join("config.json");
    if let Ok(json_content) = fs::read_to_string(config_path) {
        if let Ok(result) = serde_json::from_str(&json_content) {
            return result;
        }
    }

    Preferences::default()
}

fn take_options() -> Options {
    let options_path = get_config_dir().join("options.json");
    if let Ok(json_content) = fs::read_to_string(options_path) {
        if let Ok(result) = serde_json::from_str(&json_content) {
            return result;
        }
    }

    Options::default()
}

fn get_ascii_art() -> String {
    let options = take_options();
    let ascii_art = options.ascii;

    if let Ok(result) = fs::read_to_string(get_config_dir().join("ascii-arts").join(format!("{}.txt",ascii_art))) {
        result
    } else {
        format!("could'nt find the file: {}.txt",ascii_art).to_string()
    }
}

fn main() {
    let ascii_art = get_ascii_art();
    let config = take_config();
    let options = take_options();
    let reset_color = "\x1b[0m";

    let color = match options.color.as_str() {
        "black" => "\x1b[1;90m",
        "red" => "\x1b[1;91m",
        "green" => "\x1b[1;92m",
        "yellow" => "\x1b[1;93m",
        "blue" => "\x1b[1;94m",
        "purple" => "\x1b[1;95m",
        "cyan" => "\x1b[1;96m",
        "white" => "\x1b[1;97m",
        _ => panic!("unknown color: {}",options.color),
    };

    let mut max_line = 0;
    for line in ascii_art.lines() {
        let len = line.len();
        if len > max_line {
            max_line = len;
            continue;
        }
    }

    let mut components = config.as_vector();
    components.push(("".to_string(),"".to_string()));

    println!();
    let mut i = 0;
    for line in ascii_art.lines() {
        if i == components.len()-1 {
            println!("{}{}{}",color,line,reset_color);
        } else if i == 0 {
            println!("{}{:<width$}  {}", color, line, sh(&components[i].1), width = max_line);
            i += 1;
        } else if i == 1 {
            println!("{}{:<width$}{}  {}", color, line, reset_color,"-".repeat(sh(&components[0].1).chars().count()), width = max_line);
            i += 1;
        } else {
            println!("{}{:<width$}  {}{}: {}", color, line, &components[i].0, reset_color,sh(&components[i].1), width = max_line);
            i += 1;
        }
    }
    println!();
}
