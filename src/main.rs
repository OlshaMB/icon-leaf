use csv::Reader as CSVReader;
use serde_json::Value;
use std::{
    collections::HashMap,
    env::args,
    process::{exit, Command},
};
#[derive(Debug)]
struct Window {
    #[allow(dead_code)]
    title: String,
}
impl From<String> for Window {
    fn from(value: String) -> Self {
        Self { title: value }
    }
}
impl From<&str> for Window {
    fn from(value: &str) -> Self {
        Self {
            title: value.to_string(),
        }
    }
}
#[derive(Debug)]
struct Windows {
    w: Vec<Window>,
}
#[allow(non_camel_case_types)]
enum QueryMode {
    #[allow(clippy::upper_case_acronyms)]
    Cli,
    #[allow(unused, clippy::upper_case_acronyms)]
    Native,
}
impl Windows {
    fn query_yabai(mode: QueryMode, desktop_id: u8) -> Self {
        let mut windows = Windows { w: vec![] };
        match mode {
            QueryMode::Cli => {
                let unparsed_output = Command::new("yabai")
                    .arg("-m")
                    .arg("query")
                    .arg("--windows")
                    .arg("--space")
                    .arg(desktop_id.to_string())
                    .output()
                    .expect("Empty Output")
                    .stdout;
                let parsed_output: Value = serde_json::from_str(
                    String::from_utf8_lossy(&unparsed_output)
                        .into_owned()
                        .as_str(),
                )
                .expect("Invalid windows json : unparsed");
                for window in parsed_output.as_array().expect("Inavlid json : 1") {
                    let window_name = window
                        .get("app")
                        .expect("Inavlid window json : 2")
                        .as_str()
                        .expect("App name is not a string");
                    windows.w.push(Window::from(window_name))
                }
            }
            _ => panic!("Unsupported mode"),
        }
        windows
    }
}
#[derive(Debug)]
struct IconStore {
    i: HashMap<String, String>,
}
impl IconStore {
    fn new() -> Self {
        Self { i: HashMap::new() }
    }
    fn load(&mut self) {
        let appsdb_path = dirs::config_dir()
            .expect("Unsupported os")
            .as_path()
            .join("icon-leaf")
            .join("appsdb");
        if cfg!(target = "debug") {
            dbg!(&appsdb_path);
        }
        if !appsdb_path.exists() {
            panic!("Some folder/file doesn't exists in path")
        }
        let mut rdr = CSVReader::from_path(appsdb_path).expect("error reading csv appsdb");
        for app in rdr.records() {
            if app.is_err() {
                println!("err1");
                continue;
            }
            let app = app.unwrap();
            if app.len() != 2 {
                println!("err2");
                continue;
            }
            self.i.insert(
                app.get(0)
                    .expect("Impossible: get name of the app")
                    .to_string(),
                app.get(1)
                    .expect("Impossible: get app icon for the app")
                    .to_string(),
            );
        }
    }
    fn match_(&self, app_name: String) -> Option<&String> {
        //println!("{}=={}", app_name, "Firefox Developer Edition");
        self.i.get(&app_name)
    }
    fn match_many(&self, windows: Windows) -> Vec<String> {
        let mut matched_icons: Vec<String> = vec![];
        for window in windows.w {
            let matched_icon = self.match_(window.title);
            match matched_icon {
                Some(icon) => matched_icons.push(icon.clone()),
                None => matched_icons.push("ﬓ".to_string()),
            }
        }
        matched_icons
    }
}
struct Formatter;
impl Formatter {
    fn format(icons: Vec<String>) -> String {
        icons.join(" ")
    }
    fn print(icons: Vec<String>) {
        print!("{}", Formatter::format(icons))
    }
}
fn main() {
    // get opts for program
    let args: Vec<String> = args().collect();
    let space_id: u8;
    let _1 = args
        .get(1)
        .expect("please provide more args\n use --help to get more info")
        .as_str();
    if ["-h", "-v", "--help", "--version"].contains(&_1) {
        println!("icon-leaf v1.0.0 OlshaMB");
        println!("icon-leaf - get's app_icons for windows in some virtual desktop");
        println!("Use CONFIG_FOLDER/icon-leaf/appsdb to configure icons for apps");
        println!("USAGE: icon-leaf <desktop_id> <flags(optional)>");
        exit(1);
    } else if let Ok(space_id_parsed) = _1.parse::<u8>() {
        space_id = space_id_parsed;
    } else {
        println!("Unknown arg 1\n use --help flag");
        exit(2)
    }

    // windows query
    let windows = Windows::query_yabai(QueryMode::Cli, space_id);
    if cfg!(mode = "debug") {
        dbg!(&windows);
    }
    // db load
    let mut icon_store = IconStore::new();
    icon_store.load();
    if cfg!(target = "debug") {
        dbg!(&icon_store);
    }
    // db query
    let icons = icon_store.match_many(windows);
    if cfg!(target = "debug") {
        dbg!(&icons);
    }
    // out
    Formatter::print(icons);
}
