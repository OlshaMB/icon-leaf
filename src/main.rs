use std::{
    env::args,
    process::exit,
};
use formatter::Formatter;
use icon_store::IconStore;
use windows_query::{QueryMode, Windows};

mod windows_query;
mod icon_store;
mod formatter;

fn main() {
    // get opts for program
    let args: Vec<String> = args().collect();
    let space_id: u8;
    let first_argument = args
        .get(1)
        .expect("please provide more args\n use --help to get more info")
        .as_str();
    if ["-h", "-v", "--help", "--version"].contains(&first_argument) {
        println!("icon-leaf v1.0.0 OlshaMB");
        println!("icon-leaf - get's app_icons for windows in some virtual desktop");
        println!("Use CONFIG_FOLDER/icon-leaf/appsdb to configure icons for apps");
        println!("USAGE: icon-leaf <desktop_id> <flags(optional)>");
        exit(1);
    } else if let Ok(space_id_parsed) = first_argument.parse::<u8>() {
        space_id = space_id_parsed;
    } else {
        println!("Unknown arg 1\n use --help flag");
        exit(2)
    }

    // windows query
    let windows = Windows::query_yabai(QueryMode::Cli, space_id);
    if cfg!(debug_assertions) {
        dbg!(&windows);
    }
    // db load
    let mut icon_store = IconStore::new();
    icon_store.load();
    if cfg!(debug_assertions) {
        dbg!(&icon_store);
    }
    // db query
    let icons = icon_store.match_many(windows);
    if cfg!(debug_assertions) {
        dbg!(&icons);
    }
    // out
    Formatter::print(icons);
}
